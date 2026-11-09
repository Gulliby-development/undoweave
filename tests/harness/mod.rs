#![allow(dead_code)]

use undoweave::cli::{reads_stdin, run};

/// One row: the words after the program name, the standard input, the exit status, standard output and standard error.
pub type Row = (&'static [&'static str], &'static str, i32, &'static str, &'static str);

fn words_of(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| word.to_string()).collect()
}

/// Runs every row through `undoweave::cli::run` and compares status, standard output and standard error.
pub fn check_rows(rows: &[Row]) {
    for (number, (words, input, status, out, err)) in rows.iter().enumerate() {
        let got = run(&words_of(words), input.as_bytes());
        assert_eq!((got.status, got.out.as_str(), got.err.as_str()), (*status, *out, *err), "row {} {:?}", number + 1, words);
    }
}

/// Compares `cli::reads_stdin` with the row's status field: 1 when the command line must read standard input, 0 when not.
pub fn check_reads(rows: &[Row]) {
    for (number, (words, _, flag, _, _)) in rows.iter().enumerate() {
        assert_eq!(reads_stdin(&words_of(words)), *flag == 1, "row {} {:?}", number + 1, words);
    }
}
