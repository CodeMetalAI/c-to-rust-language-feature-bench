fn main() {
    let s: [u8; 4] = *b"abc";
    let t: [u8; 3] = *b"abc";

    if s[0]!= b'a' || s[1]!= b'b' || s[2]!= b'c' || s[3]!= b'\0' {
        return 1;
    }

    if t[0]!= b'a' || t[1]!= b'b' || t[2]!= b'c' {
        return 2;
    }

    let mut s: [u8; 4] = *b"abc";
    let mut t: [u8; 3] = *b"abc";

    s[0] = b'x';
    t[2] = b'y';

    if s[0]!= b'x' {
        return 3;
    }
    if t[2]!= b'y' {
        return 4;
    }

    {
        let p: &[u8] = b"abc";
        if p[0]!= b'a' || p[1]!= b'b' || p[2]!= b'c' || p[3]!= b'\0' {
            return 5;
        }
    }

    return 0;
}