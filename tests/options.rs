mod harness;

const ROWS: &[harness::Row] = &[
    (&["final"], "ins 0 a\n", 0, "\"a\"\n", ""),
    (&["final", "x"], "", 2, "", "undoweave: unexpected argument x\n"),
    (&["final", "-"], "", 2, "", "undoweave: unexpected argument -\n"),
    (&["final", ""], "", 2, "", "undoweave: unexpected argument \n"),
    (&["final", "--bogus"], "", 2, "", "undoweave: unknown option --bogus\n"),
    (&["check", "--coalesce"], "", 2, "", "undoweave: unknown option --coalesce\n"),
    (&["check", "--max-rev", "5"], "", 2, "", "undoweave: unknown option --max-rev\n"),
    (&["final", "--coalesce", "--coalesce"], "", 2, "", "undoweave: given twice: --coalesce\n"),
    (&["final", "--max-rev"], "", 2, "", "undoweave: missing value for --max-rev\n"),
    (&["final", "--max-rev", "0"], "", 2, "", "undoweave: bad value for --max-rev\n"),
    (&["final", "--max-rev", "41"], "", 2, "", "undoweave: bad value for --max-rev\n"),
    (&["final", "--max-rev", "100"], "", 2, "", "undoweave: bad value for --max-rev\n"),
    (&["final", "--max-rev", "07"], "ins 0 a\n", 0, "\"a\"\n", ""),
    (&["final", "--max-rev", "x"], "", 2, "", "undoweave: bad value for --max-rev\n"),
    (&["final", "--max-rev", "-1"], "", 2, "", "undoweave: bad value for --max-rev\n"),
    (&["final", "--max-rev", "5", "--max-rev", "6"], "", 2, "", "undoweave: given twice: --max-rev\n"),
    (&["final", "--max-rev", "5", "--max-rev", "0"], "", 2, "", "undoweave: bad value for --max-rev\n"),
    (&["final", "--max-rev", "--coalesce"], "", 2, "", "undoweave: bad value for --max-rev\n"),
    (&["trace", "--coalesce", "--max-rev", "3"], "ins 0 a\n", 0, "1 ins rev 1 \"a\"\nend rev 1 revisions 2\n", ""),
    (&["tree", "--max-rev", "40"], "ins 0 a\n", 0, "r0 \"\"\nr1 <- r0 \"a\" *\n", ""),
    (&["trace", "--max-rev", "1"], "ins 0 a\n", 0, "1 ins rev 1 \"a\"\nend rev 1 revisions 2\n", ""),
];

#[test]
fn rows() {
    harness::check_rows(ROWS);
}
