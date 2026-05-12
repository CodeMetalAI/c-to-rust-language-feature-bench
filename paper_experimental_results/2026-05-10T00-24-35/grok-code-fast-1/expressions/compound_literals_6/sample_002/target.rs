fn main() {
    let a_bytes = &[b'a', b'b', b'c', 0];
    let a = std::str::from_utf8(a_bytes).unwrap();
    let b = "abc";

    if a.as_bytes()[0] != b'a' || a.as_bytes()[1] != b'b' || a.as_bytes()[2] != b'c' || a.as_bytes()[3] != 0 {
        std::process::exit(1);
    }
    if b.as_bytes()[0] != b'a' || b.as_bytes()[1] != b'b' || b.as_bytes()[2] != b'c' || b.as_bytes()[3] != 0 {
        std::process::exit(2);
    }

    let equal = a == b;
    if equal && !equal {
        std::process::exit(3);
    }

    std::process::exit(0);
}