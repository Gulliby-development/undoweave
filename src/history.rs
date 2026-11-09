//! Revision-tree storage and state transitions for replay.

use std::collections::BTreeMap;

use crate::script::ParsedCommand;
use crate::{Failure, Options};

/// One immutable branch point, except that the active typing revision may grow while coalescing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Revision {
    parent: Option<usize>,
    children: Vec<usize>,
    text: Vec<u8>,
    marks: Vec<Vec<u8>>,
}

impl Revision {
    pub fn parent(&self) -> Option<usize> {
        self.parent
    }

    pub fn children(&self) -> &[usize] {
        &self.children
    }

    pub fn text(&self) -> &[u8] {
        &self.text
    }

    pub fn marks(&self) -> &[Vec<u8>] {
        &self.marks
    }
}

/// The owned revision tree and navigation state for a single script replay.
#[derive(Debug, Clone)]
pub struct History {
    revisions: Vec<Revision>,
    current: usize,
    marks: BTreeMap<Vec<u8>, usize>,
    options: Options,
    previous_insert_end: Option<usize>,
    typing_revision: Option<usize>,
}

impl History {
    pub fn new(options: Options) -> Self {
        Self {
            revisions: vec![Revision { parent: None, children: Vec::new(), text: Vec::new(), marks: Vec::new() }],
            current: 0,
            marks: BTreeMap::new(),
            options,
            previous_insert_end: None,
            typing_revision: None,
        }
    }

    /// Apply one parsed command. The returned reason is suitable for line qualification by replay.
    pub fn apply(&mut self, command: &ParsedCommand) -> Result<(), Failure> {
        if !matches!(command, ParsedCommand::Insert { .. }) {
            self.end_typing_run();
        }
        match command {
            ParsedCommand::Insert { pos, text } => self.insert(*pos, text),
            ParsedCommand::Delete { pos, len } => self.delete(*pos, *len),
            ParsedCommand::Undo => {
                let Some(parent) = self.revisions[self.current].parent else {
                    return Err(state_error("nothing to undo"));
                };
                self.current = parent;
                Ok(())
            }
            ParsedCommand::Redo { index } => {
                let children = &self.revisions[self.current].children;
                if children.is_empty() {
                    return Err(state_error("nothing to redo"));
                }
                let branch = index.unwrap_or(children.len());
                if branch == 0 || branch > children.len() {
                    return Err(state_error(format!("no branch {branch} (have {})", children.len())));
                }
                self.current = children[branch - 1];
                Ok(())
            }
            ParsedCommand::Goto { target, numeric } => {
                let destination = if let Some(number) = numeric { (*number < self.revisions.len()).then_some(*number) } else { self.marks.get(target).copied() };
                let Some(destination) = destination else {
                    return Err(state_error(format!("unknown revision {}", String::from_utf8_lossy(target))));
                };
                self.current = destination;
                Ok(())
            }
            ParsedCommand::Mark { name } => {
                if let Some(previous) = self.marks.insert(name.clone(), self.current) {
                    if previous != self.current {
                        self.revisions[previous].marks.retain(|mark| mark != name);
                    }
                }
                let revision_marks = &mut self.revisions[self.current].marks;
                if !revision_marks.iter().any(|mark| mark == name) {
                    revision_marks.push(name.clone());
                    revision_marks.sort();
                }
                Ok(())
            }
        }
    }

    fn insert(&mut self, pos: usize, text: &[u8]) -> Result<(), Failure> {
        let length = self.revisions[self.current].text.len();
        if pos > length {
            return Err(state_error(format!("position {pos} is past the end (length {length})")));
        }
        let Some(new_length) = length.checked_add(text.len()) else {
            return Err(state_error("buffer is too long (max 64 bytes)"));
        };
        if new_length > 64 {
            return Err(state_error("buffer is too long (max 64 bytes)"));
        }

        let coalesce = self.options.coalesce && self.previous_insert_end == Some(pos) && self.typing_revision == Some(self.current);
        if coalesce {
            self.revisions[self.current].text.splice(pos..pos, text.iter().copied());
        } else {
            self.ensure_revision_available()?;
            let mut new_text = self.revisions[self.current].text.clone();
            new_text.splice(pos..pos, text.iter().copied());
            self.create_revision(new_text);
        }
        self.previous_insert_end = pos.checked_add(text.len());
        self.typing_revision = Some(self.current);
        Ok(())
    }

    fn delete(&mut self, pos: usize, len: usize) -> Result<(), Failure> {
        let length = self.revisions[self.current].text.len();
        let end = pos.checked_add(len);
        if end.is_none_or(|end| end > length) {
            return Err(state_error(format!("range {pos}+{len} is past the end (length {length})")));
        }
        self.ensure_revision_available()?;
        let mut new_text = self.revisions[self.current].text.clone();
        new_text.drain(pos..end.unwrap_or(pos));
        self.create_revision(new_text);
        Ok(())
    }

    fn ensure_revision_available(&self) -> Result<(), Failure> {
        let used = self.revisions.len() - 1;
        if used >= self.options.max_rev {
            Err(state_error(format!("too many revisions (max {})", self.options.max_rev)))
        } else {
            Ok(())
        }
    }

    fn create_revision(&mut self, text: Vec<u8>) {
        let id = self.revisions.len();
        let parent = self.current;
        self.revisions.push(Revision { parent: Some(parent), children: Vec::new(), text, marks: Vec::new() });
        self.revisions[parent].children.push(id);
        self.current = id;
    }

    fn end_typing_run(&mut self) {
        self.previous_insert_end = None;
        self.typing_revision = None;
    }

    pub fn current_revision(&self) -> usize {
        self.current
    }

    pub fn current_text(&self) -> &[u8] {
        self.revisions[self.current].text()
    }

    pub fn revisions(&self) -> &[Revision] {
        &self.revisions
    }

    pub fn revision_count(&self) -> usize {
        self.revisions.len()
    }

    pub fn depth(&self) -> usize {
        let mut depth = 0;
        let mut revision = self.current;
        while let Some(parent) = self.revisions[revision].parent {
            depth += 1;
            revision = parent;
        }
        depth
    }
}

fn state_error(message: impl Into<String>) -> Failure {
    Failure::new(1, message)
}
