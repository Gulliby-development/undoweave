//! Command-line validation, stdin eligibility, and registered command dispatch.

use crate::registry::{COMMANDS, USAGE};
use crate::{Failure, Options, Outcome};

enum Invocation<'a> {
    Immediate(Outcome),
    Command(&'a crate::Command, Options),
}

fn error(status: i32, message: impl Into<String>) -> Outcome {
    let failure = Failure::new(status, message);
    Outcome { status: failure.status, out: String::new(), err: format!("undoweave: {}\n", failure.message) }
}

fn validate(args: &[String]) -> Invocation<'_> {
    let Some(first) = args.first() else {
        return Invocation::Immediate(Outcome { status: 2, out: String::new(), err: USAGE.to_string() });
    };
    if first == "help" || first == "-h" || first == "--help" {
        return Invocation::Immediate(Outcome { status: 0, out: USAGE.to_string(), err: String::new() });
    }
    if first == "--version" {
        return Invocation::Immediate(Outcome { status: 0, out: "undoweave 0.1.0\n".to_string(), err: String::new() });
    }
    let Some(command) = COMMANDS.iter().find(|command| command.name == first) else {
        return Invocation::Immediate(error(2, format!("unknown command {first}")));
    };

    let mut options = Options { coalesce: false, max_rev: 40 };
    let mut seen_coalesce = false;
    let mut seen_max_rev = false;
    let mut index = 1;
    while index < args.len() {
        let word = &args[index];
        if word.is_empty() || word == "-" || !word.starts_with('-') {
            return Invocation::Immediate(error(2, format!("unexpected argument {word}")));
        }
        if !command.options || (word != "--coalesce" && word != "--max-rev") {
            return Invocation::Immediate(error(2, format!("unknown option {word}")));
        }
        if word == "--coalesce" {
            if seen_coalesce {
                return Invocation::Immediate(error(2, "given twice: --coalesce"));
            }
            seen_coalesce = true;
            options.coalesce = true;
            index += 1;
            continue;
        }

        let Some(value) = args.get(index + 1) else {
            return Invocation::Immediate(error(2, "missing value for --max-rev"));
        };
        let digits = value.as_bytes();
        if !(1..=2).contains(&digits.len()) || !digits.iter().all(u8::is_ascii_digit) {
            return Invocation::Immediate(error(2, "bad value for --max-rev"));
        }
        let parsed = usize::from(digits[0] - b'0') * if digits.len() == 2 { 10 } else { 1 } + if digits.len() == 2 { usize::from(digits[1] - b'0') } else { 0 };
        if !(1..=40).contains(&parsed) {
            return Invocation::Immediate(error(2, "bad value for --max-rev"));
        }
        if seen_max_rev {
            return Invocation::Immediate(error(2, "given twice: --max-rev"));
        }
        seen_max_rev = true;
        options.max_rev = parsed;
        index += 2;
    }
    Invocation::Command(command, options)
}

pub fn reads_stdin(args: &[String]) -> bool {
    matches!(validate(args), Invocation::Command(_, _))
}

pub fn run(args: &[String], input: &[u8]) -> Outcome {
    match validate(args) {
        Invocation::Immediate(outcome) => outcome,
        Invocation::Command(command, options) => match (command.handler)(&options, input) {
            Ok(out) => Outcome { status: 0, out, err: String::new() },
            Err(failure) => error(failure.status, failure.message),
        },
    }
}
