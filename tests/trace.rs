mod harness;

const ROWS: &[harness::Row] = &[
    (&["trace"], "", 0, "end rev 0 revisions 1\n", ""),
    (&["trace"], "ins 0 ab\nins 2 cd\nundo\nredo\n", 0, "1 ins rev 1 \"ab\"\n2 ins rev 2 \"abcd\"\n3 undo rev 1 \"ab\"\n4 redo rev 2 \"abcd\"\nend rev 2 revisions 3\n", ""),
    (&["trace"], "# start\n\nins 0 ab\nmark v1\ndel 0 1\ngoto v1\n", 0, "3 ins rev 1 \"ab\"\n4 mark rev 1 \"ab\"\n5 del rev 2 \"b\"\n6 goto rev 1 \"ab\"\nend rev 1 revisions 3\n", ""),
    (&["trace"], "ins 0 a\nundo\nins 0 b\nundo\nredo 1\nundo\nredo 2\n", 0, "1 ins rev 1 \"a\"\n2 undo rev 0 \"\"\n3 ins rev 2 \"b\"\n4 undo rev 0 \"\"\n5 redo rev 1 \"a\"\n6 undo rev 0 \"\"\n7 redo rev 2 \"b\"\nend rev 2 revisions 3\n", ""),
    (&["trace", "--coalesce"], "ins 0 a\nins 1 b\nins 2 c\n", 0, "1 ins rev 1 \"a\"\n2 ins rev 1 \"ab\"\n3 ins rev 1 \"abc\"\nend rev 1 revisions 2\n", ""),
    (&["trace", "--coalesce"], "ins 0 a\nins 1 b\nins 0 c\n", 0, "1 ins rev 1 \"a\"\n2 ins rev 1 \"ab\"\n3 ins rev 2 \"cab\"\nend rev 2 revisions 3\n", ""),
    (&["trace", "--coalesce"], "ins 0 a\nmark m\nins 1 b\n", 0, "1 ins rev 1 \"a\"\n2 mark rev 1 \"a\"\n3 ins rev 2 \"ab\"\nend rev 2 revisions 3\n", ""),
    (&["trace", "--coalesce"], "ins 0 a\nins 1 b\nundo\nins 0 c\nins 1 d\n", 0, "1 ins rev 1 \"a\"\n2 ins rev 1 \"ab\"\n3 undo rev 0 \"\"\n4 ins rev 2 \"c\"\n5 ins rev 2 \"cd\"\nend rev 2 revisions 3\n", ""),
    (&["trace", "--coalesce"], "ins 0 a\ndel 0 1\nins 0 b\nins 1 c\n", 0, "1 ins rev 1 \"a\"\n2 del rev 2 \"\"\n3 ins rev 3 \"b\"\n4 ins rev 3 \"bc\"\nend rev 3 revisions 4\n", ""),
    (&["trace"], "ins 0 a\nins 1 b\nins 2 c\n", 0, "1 ins rev 1 \"a\"\n2 ins rev 2 \"ab\"\n3 ins rev 3 \"abc\"\nend rev 3 revisions 4\n", ""),
    (&["trace", "--coalesce", "--max-rev", "2"], "ins 0 a\nins 1 b\nins 2 c\nins 3 d\n", 0, "1 ins rev 1 \"a\"\n2 ins rev 1 \"ab\"\n3 ins rev 1 \"abc\"\n4 ins rev 1 \"abcd\"\nend rev 1 revisions 2\n", ""),
    (&["trace"], "ins 0 \\x07\\tz\\\\\n", 0, "1 ins rev 1 \"\\x07\\tz\\\\\"\nend rev 1 revisions 2\n", ""),
];

#[test]
fn rows() {
    harness::check_rows(ROWS);
}
