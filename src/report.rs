//! Replay scripts atomically and render the final, trace, tree, or check report.

use crate::escape::shown;
use crate::history::History;
use crate::script::{parse_line, Lines};
use crate::{Failure, Options, Step};

struct Replay {
    history: History,
    steps: Vec<Step>,
    commands: usize,
}

fn replay(options: &Options, input: &[u8]) -> Result<Replay, Failure> {
    let mut history = History::new(*options);
    let mut steps = Vec::new();
    let mut commands = 0;
    for (line_number, line) in Lines::new(input) {
        let parsed = parse_line(line).map_err(|failure| qualify(line_number, failure))?;
        let Some(command) = parsed else {
            continue;
        };
        history.apply(&command).map_err(|failure| qualify(line_number, failure))?;
        commands += 1;
        steps.push(Step { line: line_number, word: command.word(), rev: history.current_revision(), text: history.current_text().to_vec() });
    }
    Ok(Replay { history, steps, commands })
}

fn qualify(line: usize, failure: Failure) -> Failure {
    Failure::new(1, format!("line {line}: {}", failure.message))
}

pub fn final_command(options: &Options, input: &[u8]) -> Result<String, Failure> {
    let replayed = replay(options, input)?;
    Ok(format!("\"{}\"\n", shown(replayed.history.current_text())))
}

pub fn trace_command(options: &Options, input: &[u8]) -> Result<String, Failure> {
    let replayed = replay(options, input)?;
    let mut output = String::new();
    for step in &replayed.steps {
        output.push_str(&format!("{} {} rev {} \"{}\"\n", step.line, step.word, step.rev, shown(&step.text)));
    }
    output.push_str(&format!("end rev {} revisions {}\n", replayed.history.current_revision(), replayed.history.revision_count()));
    Ok(output)
}

pub fn tree_command(options: &Options, input: &[u8]) -> Result<String, Failure> {
    let replayed = replay(options, input)?;
    let mut output = String::new();
    for (id, revision) in replayed.history.revisions().iter().enumerate() {
        output.push_str(&format!("r{id}"));
        if let Some(parent) = revision.parent() {
            output.push_str(&format!(" <- r{parent}"));
        }
        output.push_str(&format!(" \"{}\"", shown(revision.text())));
        for mark in revision.marks() {
            output.push_str(&format!(" @{}", shown(mark)));
        }
        if id == replayed.history.current_revision() {
            output.push_str(" *");
        }
        output.push('\n');
    }
    Ok(output)
}

pub fn check_command(options: &Options, input: &[u8]) -> Result<String, Failure> {
    let replayed = replay(options, input)?;
    Ok(format!("ok commands={} revisions={} depth={}\n", replayed.commands, replayed.history.revision_count(), replayed.history.depth()))
}
