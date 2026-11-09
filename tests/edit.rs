mod harness;

const ROWS: &[harness::Row] = &[
    (&["final"], "ins 0 ab\n", 0, "\"ab\"\n", ""),
    (&["final"], "ins 0 ab\nins 1 X\n", 0, "\"aXb\"\n", ""),
    (&["final"], "ins 0 ab\nins 2 cd\n", 0, "\"abcd\"\n", ""),
    (&["final"], "ins 0 ab\nins 3 c\n", 1, "", "undoweave: line 2: position 3 is past the end (length 2)\n"),
    (&["final"], "ins 0 abc\ndel 1 1\n", 0, "\"ac\"\n", ""),
    (&["final"], "ins 0 abcd\ndel 0 4\n", 0, "\"\"\n", ""),
    (&["final"], "ins 0 abcd\ndel 3 2\n", 1, "", "undoweave: line 2: range 3+2 is past the end (length 4)\n"),
    (&["final"], "ins 0 abcd\ndel 4 1\n", 1, "", "undoweave: line 2: range 4+1 is past the end (length 4)\n"),
    (&["final"], "ins 0 ab\ndel 2 0\n", 1, "", "undoweave: line 2: bad arguments for del\n"),
    (&["final"], "del 0 1\n", 1, "", "undoweave: line 1: range 0+1 is past the end (length 0)\n"),
    (&["final"], "ins 0 a\\nb\\tc\\\\d\n", 0, "\"a\\nb\\tc\\\\d\"\n", ""),
    (&["final"], "ins 0 \\xe9\\x41\\XfF\n", 1, "", "undoweave: line 1: bad escape\n"),
    (&["final"], "ins 0  a \nins 3  b\n", 0, "\" a  b\"\n", ""),
    (&["final"], "ins 0 a b\n", 0, "\"a b\"\n", ""),
    (&["final"], "ins 0 abcdefghijklmnop\nins 16 abcdefghijklmnop\nins 32 abcdefghijklmnop\nins 48 abcdefghijklmnop\n", 0, "\"abcdefghijklmnopabcdefghijklmnopabcdefghijklmnopabcdefghijklmnop\"\n", ""),
    (&["final"], "ins 0 abcdefghijklmnop\nins 16 abcdefghijklmnop\nins 32 abcdefghijklmnop\nins 48 abcdefghijklmnop\nins 64 a\n", 1, "", "undoweave: line 5: buffer is too long (max 64 bytes)\n"),
    (&["final"], "ins 0 abcdefghijklmnop\nins 16 abcdefghijklmnop\nins 32 abcdefghijklmnop\nins 48 abcdefghijklmnop\ndel 0 1\nins 0 a\n", 0, "\"abcdefghijklmnopabcdefghijklmnopabcdefghijklmnopabcdefghijklmnop\"\n", ""),
];

#[test]
fn rows() {
    harness::check_rows(ROWS);
}
