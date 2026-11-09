//! Lazy byte-oriented script line iteration and command parsing.

use crate::escape::{shown, unescape};
use crate::Failure;

/// One syntactic command, with insertion text decoded into owned bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedCommand {
    Insert { pos: usize, text: Vec<u8> },
    Delete { pos: usize, len: usize },
    Undo,
    Redo { index: Option<usize> },
    Goto { target: Vec<u8>, numeric: Option<usize> },
    Mark { name: Vec<u8> },
}

impl ParsedCommand {
    /// The command word as used in trace output and diagnostics.
    pub fn word(&self) -> &'static str {
        match self {
            Self::Insert { .. } => "ins",
            Self::Delete { .. } => "del",
            Self::Undo => "undo",
            Self::Redo { .. } => "redo",
            Self::Goto { .. } => "goto",
            Self::Mark { .. } => "mark",
        }
    }
}

/// A numbered view into a script, yielding lines only as the caller requests them.
pub struct Lines<'a> {
    input: &'a [u8],
    offset: usize,
    number: usize,
}

impl<'a> Lines<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { input, offset: 0, number: 0 }
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = (usize, &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.input.len() {
            return None;
        }
        let start = self.offset;
        let end = self.input[start..].iter().position(|&byte| byte == b'\n').map(|delta| start + delta);
        let mut line_end = end.unwrap_or(self.input.len());
        self.offset = end.map_or(self.input.len(), |index| index + 1);
        if line_end > start && self.input[line_end - 1] == b'\r' {
            line_end -= 1;
        }
        self.number += 1;
        Some((self.number, &self.input[start..line_end]))
    }
}

/// Parse a single numbered line. Skipped lines return no command; reasons are unqualified.
pub fn parse_line(line: &[u8]) -> Result<Option<ParsedCommand>, Failure> {
    if line.is_empty() || line.first() == Some(&b'#') {
        return Ok(None);
    }
    if line.first() == Some(&b' ') {
        return Err(Failure::new(1, "bad line"));
    }

    let command_end = line.iter().position(|&byte| byte == b' ').unwrap_or(line.len());
    let command = &line[..command_end];
    let has_separator = command_end < line.len();
    let args = if has_separator { &line[command_end + 1..] } else { &line[line.len()..] };
    let word = shown(command);

    let parsed = match command {
        b"ins" => parse_insert(args)?,
        b"del" => {
            let fields = exact_fields(args, 2).ok_or_else(|| bad_args(&word))?;
            let pos = decimal(fields[0], 1, 4).ok_or_else(|| bad_args(&word))?;
            let len = decimal(fields[1], 1, 4).filter(|&value| value > 0).ok_or_else(|| bad_args(&word))?;
            ParsedCommand::Delete { pos, len }
        }
        b"undo" => {
            if has_separator {
                return Err(bad_args(&word));
            }
            ParsedCommand::Undo
        }
        b"redo" => {
            if !has_separator {
                ParsedCommand::Redo { index: None }
            } else {
                let index = decimal(args, 1, 3).filter(|&value| value > 0).ok_or_else(|| bad_args(&word))?;
                ParsedCommand::Redo { index: Some(index) }
            }
        }
        b"goto" => {
            if args.is_empty() || args.len() > 8 || !args.iter().all(|&byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()) {
                return Err(bad_args(&word));
            }
            let numeric = if args.iter().all(u8::is_ascii_digit) { decimal(args, 1, 8) } else { None };
            ParsedCommand::Goto { target: args.to_vec(), numeric }
        }
        b"mark" => {
            if args.is_empty() || args.contains(&b' ') {
                return Err(bad_args(&word));
            }
            let valid_chars = args.len() <= 8 && args.iter().all(|&byte| byte.is_ascii_lowercase() || byte.is_ascii_digit());
            let has_letter = args.iter().any(u8::is_ascii_lowercase);
            if !valid_chars || !has_letter {
                return Err(Failure::new(1, format!("bad mark name {}", shown(args))));
            }
            ParsedCommand::Mark { name: args.to_vec() }
        }
        _ => return Err(Failure::new(1, format!("unknown command {word}"))),
    };
    Ok(Some(parsed))
}

fn parse_insert(args: &[u8]) -> Result<ParsedCommand, Failure> {
    let invalid = || Failure::new(1, "bad arguments for ins");
    let Some(separator) = args.iter().position(|&byte| byte == b' ') else {
        return Err(invalid());
    };
    let pos = decimal(&args[..separator], 1, 4).ok_or_else(invalid)?;
    let raw_text = &args[separator + 1..];
    if raw_text.is_empty() {
        return Err(invalid());
    }
    // Validate the complete escape stream before applying its decoded-size limit.
    let text = unescape(raw_text)?;
    if text.is_empty() {
        return Err(invalid());
    }
    if text.len() > 16 {
        return Err(Failure::new(1, "text is too long (max 16 bytes)"));
    }
    Ok(ParsedCommand::Insert { pos, text })
}

fn exact_fields(input: &[u8], count: usize) -> Option<Vec<&[u8]>> {
    let fields: Vec<_> = input.split(|&byte| byte == b' ').collect();
    if fields.len() == count && fields.iter().all(|field| !field.is_empty()) {
        Some(fields)
    } else {
        None
    }
}

fn decimal(input: &[u8], min_digits: usize, max_digits: usize) -> Option<usize> {
    if !(min_digits..=max_digits).contains(&input.len()) || !input.iter().all(u8::is_ascii_digit) {
        return None;
    }
    input.iter().try_fold(0usize, |value, byte| value.checked_mul(10)?.checked_add(usize::from(byte - b'0')))
}

fn bad_args(word: &str) -> Failure {
    Failure::new(1, format!("bad arguments for {word}"))
}
