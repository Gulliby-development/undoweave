mod harness;

const ROWS: &[harness::Row] = &[
    (&["final"], "ins 0 a\nundo\n", 0, "\"\"\n", ""),
    (&["final"], "ins 0 a\nundo\nredo\n", 0, "\"a\"\n", ""),
    (&["final"], "undo\n", 1, "", "undoweave: line 1: nothing to undo\n"),
    (&["final"], "redo\n", 1, "", "undoweave: line 1: nothing to redo\n"),
    (&["final"], "ins 0 a\nredo\n", 1, "", "undoweave: line 2: nothing to redo\n"),
    (&["final"], "ins 0 a\nundo\nins 0 b\nundo\nredo\n", 0, "\"b\"\n", ""),
    (&["final"], "ins 0 a\nundo\nins 0 b\nundo\nredo 1\n", 0, "\"a\"\n", ""),
    (&["final"], "ins 0 a\nundo\nins 0 b\nundo\nredo 2\n", 0, "\"b\"\n", ""),
    (&["final"], "ins 0 a\nundo\nins 0 b\nundo\nredo 3\n", 1, "", "undoweave: line 5: no branch 3 (have 2)\n"),
    (&["final"], "ins 0 a\nundo\nredo 0\n", 1, "", "undoweave: line 3: bad arguments for redo\n"),
    (&["final"], "ins 0 a\nins 1 b\ngoto 1\n", 0, "\"a\"\n", ""),
    (&["final"], "ins 0 a\nins 1 b\ngoto 0\n", 0, "\"\"\n", ""),
    (&["final"], "ins 0 a\nins 1 b\ngoto 3\n", 1, "", "undoweave: line 3: unknown revision 3\n"),
    (&["final"], "ins 0 a\nmark one\nins 1 b\ngoto one\n", 0, "\"a\"\n", ""),
    (&["final"], "ins 0 a\ngoto one\n", 1, "", "undoweave: line 2: unknown revision one\n"),
    (&["final"], "ins 0 a\nmark one\nins 1 b\nmark one\ngoto one\n", 0, "\"ab\"\n", ""),
    (&["final"], "ins 0 a\nins 1 b\nmark abcdefgh\ngoto 0\ngoto abcdefgh\n", 0, "\"ab\"\n", ""),
    (&["final"], "ins 0 a\nins 1 b\nins 2 c\n", 0, "\"abc\"\n", ""),
    (&["final", "--max-rev", "2"], "ins 0 a\nins 1 b\n", 0, "\"ab\"\n", ""),
    (&["final", "--max-rev", "2"], "ins 0 a\nins 1 b\nins 2 c\n", 1, "", "undoweave: line 3: too many revisions (max 2)\n"),
    (&["final", "--max-rev", "2"], "ins 0 a\nins 1 b\nundo\nundo\nredo\nredo\nundo\nins 1 c\n", 1, "", "undoweave: line 8: too many revisions (max 2)\n"),
    (&["final", "--max-rev", "1"], "ins 0 a\nundo\nins 0 b\n", 1, "", "undoweave: line 3: too many revisions (max 1)\n"),
];

#[test]
fn rows() {
    harness::check_rows(ROWS);
}
