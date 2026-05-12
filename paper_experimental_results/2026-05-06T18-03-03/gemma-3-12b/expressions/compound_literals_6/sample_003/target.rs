fn main() {
    let a: &[u8] = b"abc";
    let b: &str = "abc";

    if a[0] as char != 'a' || a[1] as char != 'b' || a[2] as char != 'c' || a[3] == 0 {
        return 1;
    }
    if b.as_bytes()[0] as char != 'a' || b.as_bytes()[1] as char != 'b' || b.as_bytes()[2] as char != 'c' || b.as_bytes()[3] == 0 {
        return 2;
    }

    if (a == b) != 0 && (a == b) != 1 {
        return 3;
    }

    0
}