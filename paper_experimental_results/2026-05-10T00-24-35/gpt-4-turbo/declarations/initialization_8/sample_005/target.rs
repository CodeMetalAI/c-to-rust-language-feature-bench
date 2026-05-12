fn main() {
    // Rust equivalent of C's char s[] = "abc";
    let mut s = ['a', 'b', 'c', '\0'];
    // Rust equivalent of C's char t[3] = "abc";
    let mut t = ['a', 'b', 'c'];

    if s[0] != 'a' || s[1] != 'b' || s[2] != 'c' || s[3] != '\0' {
        std::process::exit(1);
    }

    if t[0] != 'a' || t[1] != 'b' || t[2] != 'c' {
        std::process::exit(2);
    }

    s[0] = 'x';
    t[2] = 'y';

    if s[0] != 'x' {
        std::process::exit(3);
    }
    if t[2] != 'y' {
        std::process::exit(4);
    }

    {
        let p = "abc";  // Rust equivalent of char *p = "abc"; (string slices are immutable)
        if p.as_bytes()[0] != b'a' || p.as_bytes()[1] != b'b' || p.as_bytes()[2] != b'c' || p.len() != 3 {
            std::process::exit(5);
        }
    }
}