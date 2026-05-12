fn main() -> i32 {
    let mut s = ["a", "b", "c"];
    let mut t = ["a", "b", "c"];

    if s[0] != "a" || s[1] != "b" || s[2] != "c" {
        return 1;
    }

    if t[0] != "a" || t[1] != "b" || t[2] != "c" {
        return 2;
    }

    s[0] = "x";
    t[2] = "y";

    if s[0] != "x" {
        return 3;
    }
    if t[2] != "y" {
        return 4;
    }

    {
        let p = "abc";
        if p.as_bytes()[0] != b'a' || p.as_bytes()[1] != b'b' || p.as_bytes()[2] != b'c' {
            return 5;
        }
    }

    return 0;
}