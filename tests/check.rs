mod harness;

const ROWS: &[harness::Row] = &[
    (&["check"], "", 0, "ok commands=0 revisions=1 depth=0\n", ""),
    (&["check"], "ins 0 a\n", 0, "ok commands=1 revisions=2 depth=1\n", ""),
    (&["check"], "# c\nins 0 ab\n\nins 2 c\nundo\nmark x\n", 0, "ok commands=4 revisions=3 depth=1\n", ""),
    (&["check"], "ins 0 a\nins 1 b\nundo\nundo\nredo 1\n", 0, "ok commands=5 revisions=3 depth=1\n", ""),
    (&["check"], "ins 0 a\nundo\nins 0 b\nins 1 c\ngoto 1\n", 0, "ok commands=5 revisions=4 depth=1\n", ""),
    (&["check"], "ins 0 a\nundo\nundo\n", 1, "", "undoweave: line 3: nothing to undo\n"),
    (&["check"], "ins 0 a\nins 5 b\n", 1, "", "undoweave: line 2: position 5 is past the end (length 1)\n"),
    (&["check"], "ins 0 a\nfrob\nins 9 b\n", 1, "", "undoweave: line 2: unknown command frob\n"),
];

#[test]
fn rows() {
    harness::check_rows(ROWS);
}
