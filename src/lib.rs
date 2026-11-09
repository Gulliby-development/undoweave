//! undoweave: replay an edit script against a text buffer that keeps an undo tree.

pub mod cli;
pub mod escape;
pub mod history;
pub mod registry;
pub mod report;
pub mod script;

/// What one run of the program gives back: the exit status and the two output streams.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub status: i32,
    pub out: String,
    pub err: String,
}

/// An error that ends a command: `cli::run` turns it into `undoweave: MESSAGE` on standard error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    pub status: i32,
    pub message: String,
}

impl Failure {
    pub fn new(status: i32, message: impl Into<String>) -> Failure {
        Failure { status, message: message.into() }
    }
}

/// The options of a command line: `--coalesce` and `--max-rev N` (at most 40 revisions, 40 when not given).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Options {
    pub coalesce: bool,
    pub max_rev: usize,
}

/// A command handler reads the whole script and returns the text for standard output.
pub type Handler = fn(&Options, &[u8]) -> Result<String, Failure>;

/// One row of the command table in `registry`: `options` says whether the command takes `--coalesce` and `--max-rev`.
pub struct Command {
    pub name: &'static str,
    pub options: bool,
    pub handler: Handler,
}

/// What a replay of a script gives back to the handlers: every executed line with the history after it.
pub struct Step {
    pub line: usize,
    pub word: &'static str,
    pub rev: usize,
    pub text: Vec<u8>,
}
