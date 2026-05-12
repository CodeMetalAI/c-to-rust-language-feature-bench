fn main() {
    let mut s: [i8; 4] = [b'a' as i8, b'b' as i8, b'c' as i8, 0];
    let mut t: [i8; 3] = [b'a' as i8, b'b' as i8, b'c' as i8];

    if s[0] != b'a' as i8 || s[1] != b'b' as i8 || s[2] != b'c' as i8 || s[3] != 0 {
        std::process::exit(1);
    }

    if t[0] != b'a' as i8 || t[1] != b'b' as i8 || t[2] != b'c' as i8 {
        std::process::exit(2);
    }

    s[0] = b'x' as i8;
    t[2] = b'y' as i8;

    if s[0] != b'x' as i8 {
        std::process::exit(3);
    }
    if t[2] != b'y' as i8 {
        std::process::exit(4);
    }

    {
        let p: [i8; 4] = [b'a' as i8, b'b' as i8, b'c' as i8, 0];
        if p[0] != b'a' as i8 || p[1] != b'b' as i8 || p[2] != b'c' as i8 || p[3] != 0 {
            std::process::exit(5);
        }
    }

    std::process::exit(0);
}