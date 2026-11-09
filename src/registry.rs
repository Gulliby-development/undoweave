//! The usage text and the command table.

use crate::report;
use crate::Command;

pub const USAGE: &str = "usage: undoweave final [--coalesce] [--max-rev N]\n       undoweave trace [--coalesce] [--max-rev N]\n       undoweave tree [--coalesce] [--max-rev N]\n       undoweave check\n";

pub const COMMANDS: &[Command] = &[
    Command { name: "final", options: true, handler: report::final_command },
    Command { name: "trace", options: true, handler: report::trace_command },
    Command { name: "tree", options: true, handler: report::tree_command },
    Command { name: "check", options: false, handler: report::check_command },
];
