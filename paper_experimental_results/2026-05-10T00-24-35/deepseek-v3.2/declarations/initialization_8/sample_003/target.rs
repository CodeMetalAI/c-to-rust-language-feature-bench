fn main() {
    let mut s = ['a' as u8, 'b' as u8, 'c' as u8, 0];
    let mut t = ['a' as u8, 'b' as u8, 'c' as u8];

    if s[0] != b'a' || s[1] != b'b' || s[2] != b'c' || s[3] != b'\0' {
        std::process::exit(1);
    }

    if t[0] != b'a' || t[1] != b'b' || t[2] != b'c' {
        std::process::exit(2);
    }

    s[0] = b'x';
    t[2] = b'y';

    if s[0] != b'x' {
        std::process::exit(3);
    }
    if t[2] != b'y' {
        std::process::exit(4);
    }

    {
        let p = "abc".as_bytes();
        if p[0] != b'a' || p[1] != b'b' || p[2] != b'c' || p[3] != b'\0' {
            std::process::exit(5);
        }
    }

    std::process::exit(0);
}