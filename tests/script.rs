mod harness;

const ROWS: &[harness::Row] = &[
    (&["check"], "", 0, "ok commands=0 revisions=1 depth=0\n", ""),
    (&["check"], "\n# note\nins 0 a\n\nfrob\n", 1, "", "undoweave: line 5: unknown command frob\n"),
    (&["check"], " ins 0 a\n", 1, "", "undoweave: line 1: bad line\n"),
    (&["check"], "ins\n", 1, "", "undoweave: line 1: bad arguments for ins\n"),
    (&["check"], "ins 0\n", 1, "", "undoweave: line 1: bad arguments for ins\n"),
    (&["check"], "ins 0 \n", 1, "", "undoweave: line 1: bad arguments for ins\n"),
    (&["check"], "ins x a\n", 1, "", "undoweave: line 1: bad arguments for ins\n"),
    (&["check"], "ins 00000 a\n", 1, "", "undoweave: line 1: bad arguments for ins\n"),
    (&["check"], "ins 0000 a\n", 0, "ok commands=1 revisions=2 depth=1\n", ""),
    (&["check"], "ins 0 a\\q\n", 1, "", "undoweave: line 1: bad escape\n"),
    (&["check"], "ins 0 a\\\n", 1, "", "undoweave: line 1: bad escape\n"),
    (&["check"], "ins 0 a\\x4\n", 1, "", "undoweave: line 1: bad escape\n"),
    (&["check"], "ins 0 abcdefghijklmnop\n", 0, "ok commands=1 revisions=2 depth=1\n", ""),
    (&["check"], "ins 0 abcdefghijklmnopq\n", 1, "", "undoweave: line 1: text is too long (max 16 bytes)\n"),
    (&["check"], "ins 0 \\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\n", 0, "ok commands=1 revisions=2 depth=1\n", ""),
    (&["check"], "ins 0 \\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\\x41\n", 1, "", "undoweave: line 1: text is too long (max 16 bytes)\n"),
    (&["check"], "Ins 0 a\n", 1, "", "undoweave: line 1: unknown command Ins\n"),
    (&["check"], "ins 0 a\nundo x\n", 1, "", "undoweave: line 2: bad arguments for undo\n"),
    (&["check"], "redo 0\n", 1, "", "undoweave: line 1: bad arguments for redo\n"),
    (&["check"], "redo x\n", 1, "", "undoweave: line 1: bad arguments for redo\n"),
    (&["check"], "goto ABC\n", 1, "", "undoweave: line 1: bad arguments for goto\n"),
    (&["check"], "goto abcdefghi\n", 1, "", "undoweave: line 1: bad arguments for goto\n"),
    (&["check"], "mark A\n", 1, "", "undoweave: line 1: bad mark name A\n"),
    (&["check"], "mark 7\n", 1, "", "undoweave: line 1: bad mark name 7\n"),
    (&["check"], "mark a b\n", 1, "", "undoweave: line 1: bad arguments for mark\n"),
    (&["check"], "ins 0 a\r\nmark q\r\n", 0, "ok commands=2 revisions=2 depth=1\n", ""),
    (&["check"], "ins 0 a\r", 0, "ok commands=1 revisions=2 depth=1\n", ""),
    (&["check"], "ins 0 a\n\n", 0, "ok commands=1 revisions=2 depth=1\n", ""),
];

#[test]
fn rows() {
    harness::check_rows(ROWS);
}
