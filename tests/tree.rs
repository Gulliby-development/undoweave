mod harness;

const ROWS: &[harness::Row] = &[
    (&["tree"], "", 0, "r0 \"\" *\n", ""),
    (&["tree"], "ins 0 ab\n", 0, "r0 \"\"\nr1 <- r0 \"ab\" *\n", ""),
    (&["tree"], "ins 0 ab\nins 2 cd\nundo\n", 0, "r0 \"\"\nr1 <- r0 \"ab\" *\nr2 <- r1 \"abcd\"\n", ""),
    (&["tree"], "ins 0 a\nundo\nins 0 b\nundo\nins 0 c\n", 0, "r0 \"\"\nr1 <- r0 \"a\"\nr2 <- r0 \"b\"\nr3 <- r0 \"c\" *\n", ""),
    (&["tree"], "ins 0 a\nmark b\nmark a\nins 1 c\nmark b\n", 0, "r0 \"\"\nr1 <- r0 \"a\" @a\nr2 <- r1 \"ac\" @b *\n", ""),
    (&["tree"], "ins 0 a\nmark z\nmark x\nmark m\n", 0, "r0 \"\"\nr1 <- r0 \"a\" @m @x @z *\n", ""),
    (&["tree"], "ins 0 a\nundo\nmark root\nredo\n", 0, "r0 \"\" @root\nr1 <- r0 \"a\" *\n", ""),
    (&["tree", "--coalesce"], "ins 0 a\nins 1 b\nins 2 c\ndel 0 1\n", 0, "r0 \"\"\nr1 <- r0 \"abc\"\nr2 <- r1 \"bc\" *\n", ""),
    (&["tree"], "ins 0 a\\nb\nins 3 \\x7f\ndel 0 1\n", 0, "r0 \"\"\nr1 <- r0 \"a\\nb\"\nr2 <- r1 \"a\\nb\\x7f\"\nr3 <- r2 \"\\nb\\x7f\" *\n", ""),
    (&["tree"], "ins 0 a\nins 1 b\ngoto 1\n", 0, "r0 \"\"\nr1 <- r0 \"a\" *\nr2 <- r1 \"ab\"\n", ""),
];

#[test]
fn rows() {
    harness::check_rows(ROWS);
}
