fn main() {
    let a = "abc";
    let b = "abc";

    if a.as_bytes()[0] != b'a' || a.as_bytes()[1] != b'b' || a.as_bytes()[2] != b'c' || a.as_bytes()[3] != b'\0' {
        std::process::exit(1);
    }
    if b.as_bytes()[0] != b'a' || b.as_bytes()[1] != b'b' || b.as_bytes()[2] != b'c' || b.as_bytes()[3] != b'\0' {
        std::process::exit(2);
    }

    if (a == b) != true && (a == b) != false {
        std::process::exit(3);
    }

    std::process::exit(0);
}