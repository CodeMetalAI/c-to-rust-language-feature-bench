use std::process::exit;

fn main() {
    let mut s: [u8; 4] = *b"abc\0";
    let mut t: [u8; 3] = *b"abc";

    if s[0] != b'a' || s[1] != b'b' || s[2] != b'c' || s[3] != b'\0' {
        exit(1);
    }

    if t[0] != b'a' || t[1] != b'b' || t[2] != b'c' {
        exit(2);
    }

    s[0] = b'x';
    t[2] = b'y';

    if s[0] != b'x' {
        exit(3);
    }
    if t[2] != b'y' {
        exit(4);
    }

    {
        let p = b"abc\0";
        if p[0] != b'a' || p[1] != b'b' || p[2] != b'c' || p[3] != b'\0' {
            exit(5);
        }
    }

    exit(0);
}