mod harness;

const ROWS: &[harness::Row] = &[
    (&[], "", 2, "", "usage: undoweave final [--coalesce] [--max-rev N]\n       undoweave trace [--coalesce] [--max-rev N]\n       undoweave tree [--coalesce] [--max-rev N]\n       undoweave check\n"),
    (&["help"], "", 0, "usage: undoweave final [--coalesce] [--max-rev N]\n       undoweave trace [--coalesce] [--max-rev N]\n       undoweave tree [--coalesce] [--max-rev N]\n       undoweave check\n", ""),
    (&["-h", "extra"], "", 0, "usage: undoweave final [--coalesce] [--max-rev N]\n       undoweave trace [--coalesce] [--max-rev N]\n       undoweave tree [--coalesce] [--max-rev N]\n       undoweave check\n", ""),
    (&["--help"], "", 0, "usage: undoweave final [--coalesce] [--max-rev N]\n       undoweave trace [--coalesce] [--max-rev N]\n       undoweave tree [--coalesce] [--max-rev N]\n       undoweave check\n", ""),
    (&["--version"], "", 0, "undoweave 0.1.0\n", ""),
    (&["--version", "x"], "", 0, "undoweave 0.1.0\n", ""),
    (&["frob"], "", 2, "", "undoweave: unknown command frob\n"),
    (&[""], "", 2, "", "undoweave: unknown command \n"),
    (&["Final"], "", 2, "", "undoweave: unknown command Final\n"),
    (&["--coalesce"], "", 2, "", "undoweave: unknown command --coalesce\n"),
    (&["help", "final"], "", 0, "usage: undoweave final [--coalesce] [--max-rev N]\n       undoweave trace [--coalesce] [--max-rev N]\n       undoweave tree [--coalesce] [--max-rev N]\n       undoweave check\n", ""),
    (&["tree", "--help"], "", 2, "", "undoweave: unknown option --help\n"),
];

#[test]
fn rows() {
    harness::check_rows(ROWS);
}
