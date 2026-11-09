mod harness;

const ROWS: &[harness::Row] = &[
    (&["final"], "", 1, "", ""),
    (&["check"], "", 1, "", ""),
    (&["trace", "--coalesce"], "", 1, "", ""),
    (&["tree", "--max-rev", "7", "--coalesce"], "", 1, "", ""),
    (&["help"], "", 0, "", ""),
    (&[], "", 0, "", ""),
    (&["frob"], "", 0, "", ""),
    (&["final", "--max-rev"], "", 0, "", ""),
    (&["check", "--coalesce"], "", 0, "", ""),
    (&["final", "x"], "", 0, "", ""),
    (&["final", "--max-rev", "41"], "", 0, "", ""),
    (&["trace", "--coalesce", "--coalesce"], "", 0, "", ""),
];

#[test]
fn rows() {
    harness::check_reads(ROWS);
}
