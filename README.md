<p align="center"><img src="docs/logo.svg" width="96" alt="" /></p>

# undoweave

[![CI](https://github.com/Gulliby-development/undoweave/actions/workflows/ci.yml/badge.svg)](https://github.com/Gulliby-development/undoweave/actions/workflows/ci.yml) [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

undoweave is a standard-library-only Rust library and command-line tool for replaying a byte-oriented edit script against a text buffer while retaining an undo history as a branching tree. It is intended for people building editors, text widgets, and collaborative-editing tests, and for use in scripts and CI. It reads scripts from standard input, never changes files, and requires no network access at runtime.

## Build and run

Install a stable Rust toolchain and Cargo, then build from the repository directory:

```sh
cargo build
```

This project uses Cargo and stable Rust on Windows, macOS, and Linux. To run the program with a script piped to it:

```sh
cargo run -- final
printf 'ins 0 hello\nins 5 !\n' | cargo run --quiet -- final
```

The first command reads the script from standard input. Cargo's `--` separates Cargo options from undoweave arguments. The built executable is `target/debug/undoweave` (on Windows, Cargo adds the platform executable suffix). The examples below use the name `undoweave` to mean that built program. In a POSIX shell, after building in the repository directory, define it as:

```sh
undoweave() { target/debug/undoweave "$@"; }
```

The function definition is for POSIX shells; use Cargo commands or the platform executable path in other shells.

## Commands

```text
usage: undoweave final [--coalesce] [--max-rev N]
       undoweave trace [--coalesce] [--max-rev N]
       undoweave tree [--coalesce] [--max-rev N]
       undoweave check
```

All commands read the script from standard input. `final` prints the current buffer in double quotes and a line feed; an empty script prints `""`. `trace` prints one snapshot per executed command and then `end rev R revisions M`, where R is the current revision and M counts revision 0. Its empty-script result is `end rev 0 revisions 1`. `tree` prints one line per revision in creation order: `rN`, optionally ` <- rP` for its parent, a space and the text in double quotes, each mark as ` @NAME`, and ` *` on the current revision. With empty input it prints `r0 "" *`. `check` prints `ok commands=C revisions=M depth=D`: C is the number of commands run, M is the revision count including revision 0, and D is the number of parent steps from the current revision back to revision 0. With empty input it prints `ok commands=0 revisions=1 depth=0`. All report lines end with a line feed.

The default revision limit is 40. `--max-rev N` sets it to 1–40; N must contain one or two decimal digits, and leading zeroes are accepted. Revision zero counts in reports, but not against this edit limit. `--coalesce` combines contiguous typing as described below. These options apply to `final`, `trace`, and `tree`; `check` takes no options.

With no command, the usage synopsis is written to standard error and the process exits with status 2. The first word `help`, `-h`, or `--help` prints usage to standard output and exits 0; `--version` prints `undoweave 0.1.0` and exits 0. Those help and version forms ignore later words. An unrecognized command is an error.

For options, arguments are validated from left to right. A non-option argument (including an empty argument or `-`) is reported as `unexpected argument WORD`. An unsupported option, or any option on `check`, is `unknown option WORD`. A repeated `--coalesce` is `given twice: --coalesce`; a final `--max-rev` without a following word is `missing value for --max-rev`. The next word is always treated as the value, even if it resembles an option. An invalid value is `bad value for --max-rev`; the value is checked before reporting a duplicate `--max-rev`. Command-line errors exit 2 and do not read the script. For valid command lines, the program reads standard input; help, version, and invalid invocations do not.

A successful command writes only its result to standard output and exits 0. A failure writes no standard output, writes one line beginning `undoweave: ` to standard error, and exits 1 for a script/replay error or 2 for a command-line error.

## Script format

Input is bytes, not Unicode text. LF separates lines; one CR immediately before LF or at the end of the input is removed. A final line without LF is processed, while a trailing LF does not create an extra line. Empty lines and lines whose first byte is `#` are ignored, but still count for line numbering. Other lines must start with a command word at the beginning of the line. Arguments are separated by exactly one space; repeated spaces therefore create empty fields, except that insertion text may itself contain spaces.

Insertion and deletion offsets are zero-based byte positions. The buffer begins empty. The six script commands are:

- `ins POS TEXT` inserts decoded TEXT at POS. POS is one to four decimal digits, including leading zeroes. TEXT must have at least one byte before escape decoding and must decode to 1–16 bytes. It is everything after the single separator following POS, so it may contain spaces.
- `del POS LEN` removes LEN bytes beginning at POS. POS and LEN each have one to four decimal digits; LEN must be positive.
- `undo` moves to the current revision's parent.
- `redo` moves to the newest child. `redo N` selects the Nth child in creation order; N is one to three decimal digits and positive.
- `goto TARGET` moves to a revision number or mark. TARGET is one to eight lowercase letters or digits. An all-digit target is a revision number; any target containing a letter is a mark name.
- `mark NAME` assigns a name to the current revision. A name is one to eight lowercase letters/digits and must contain a letter. Reusing a name moves that mark to the current revision.

In insertion text, the escape sequences `\n`, `\t`, `\\`, and `\xHH` represent a line feed byte, tab byte, backslash byte, and a byte written with two hexadecimal digits, respectively. Hex digits may use either case. Any other backslash sequence, including a trailing backslash, is a bad escape. Every other input byte represents itself.

Displayed buffer text is ASCII. A backslash, line feed, and tab are shown as `\\`, `\n`, and `\t`; other bytes below 0x20 or above 0x7E appear as `\xhh` with lowercase hexadecimal digits. Printable ASCII bytes appear unchanged. Reports surround shown text with double quotes; those quotes are delimiters, not part of the buffer.

## History, coalescing, and limits

Revision 0 contains the empty buffer. Each edit normally creates the next monotonically numbered revision as a child of the current revision, preserving all other branches and their text. Undo, redo, goto, and mark do not create revisions. Redo without an index chooses the most recently created child. Marks are movable names; tree output lists marks on each revision in bytewise name order.

With `--coalesce`, an insertion joins the previous insertion's revision only when the previous command that ran was an insertion and the new position equals the end position of that insertion. Blank lines and comments do not interrupt a run; every other executed command does, including mark and goto. An insertion at a different position starts a new revision and a new run. Coalescing still observes position and buffer-size checks, but a joined insertion does not consume another revision. Trace entries retain the text snapshot from the time each command ran.

Insertion text is limited to 16 decoded bytes and the buffer to 64 bytes. Positions and ranges must fit in the current buffer. The revision limit applies to newly created edits; a coalesced insertion does not count as a new revision. Replay processes lines in order and stops at the first failure, so later errors are not examined. No partial report is returned after a failure.

Script errors exit 1 and have the form `undoweave: line N: REASON`. For each line, checks occur in this order: first the line shape and argument validation (including mark-name validation); for `ins`, escape validation and then the decoded text-length limit; state checks such as position, buffer/range bounds, undo/redo/goto availability; finally, for an edit that would create a revision, the revision limit. Thus a bad position or oversized buffer/range is reported before `too many revisions (max M)`. Replay stops at the first failing line.

The possible script reasons are:

- `bad line`; `unknown command WORD`; `bad arguments for WORD`; `bad mark name NAME`
- `bad escape`; `text is too long (max 16 bytes)`
- `position POS is past the end (length L)`; `buffer is too long (max 64 bytes)`; `range POS+LEN is past the end (length L)`
- `nothing to undo`; `nothing to redo`; `no branch N (have K)`; `unknown revision TARGET`
- `too many revisions (max M)`

Unknown command words and invalid mark names in diagnostics use the same ASCII shown-text escaping described above. Numeric positions and lengths in state diagnostics are rendered as decimal values without leading zeroes. Command-line diagnostics are prefixed with `undoweave: ` and include `unexpected argument WORD`, `unknown option WORD`, `given twice: --coalesce`, `given twice: --max-rev`, `missing value for --max-rev`, or `bad value for --max-rev`. For `--max-rev`, its value is validated before a duplicate-option error is reported.

## Examples

These are POSIX-shell transcripts from the repository directory after `cargo build`, with the `undoweave` function defined above:

```text
$ printf 'ins 0 hello\nins 5 !\n' | undoweave final
"hello!"
$ printf 'ins 0 ab\nins 2 cd\nundo\nins 2 X\nmark b\n' | undoweave tree
r0 ""
r1 <- r0 "ab"
r2 <- r1 "abcd"
r3 <- r1 "abX" @b *
$ printf 'ins 0 ab\nins 2 cd\n' | undoweave trace --coalesce; printf 'ins 0 ab\nmark m\nins 2 cd\n' | undoweave trace --coalesce
1 ins rev 1 "ab"
2 ins rev 1 "abcd"
end rev 1 revisions 2
1 ins rev 1 "ab"
2 mark rev 1 "ab"
3 ins rev 2 "abcd"
end rev 2 revisions 3
$ printf '%s\n' 'ins 0 a\tb\x41' | undoweave final
"a\tbA"
```

## Development

The library exposes the command-line interface, byte escaping, script parsing, history, reports, and the command registry through modules in `src/`. The binary entry point reads standard input only for valid command invocations and writes the returned outcome to the process streams.

Format, lint, and test changes with:

```sh
cargo fmt
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The project is licensed under the [MIT License](LICENSE).

---

Built with [BAKED](https://github.com/Gulliby-development/baked), an open-source coding agent (bench round R92).
