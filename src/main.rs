use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut input = Vec::new();
    if undoweave::cli::reads_stdin(&args) {
        std::io::stdin().read_to_end(&mut input).expect("cannot read standard input");
    }
    let outcome = undoweave::cli::run(&args, &input);
    std::io::stdout().write_all(outcome.out.as_bytes()).expect("cannot write standard output");
    std::io::stderr().write_all(outcome.err.as_bytes()).expect("cannot write standard error");
    std::process::exit(outcome.status);
}
