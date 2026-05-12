fn main() -> i32 {
    let mut s = [b'a', b'b', b'c', b'\0'];
    let t = [b'a', b'b', b'c'];

    if s[0] != b'a' || s[1] != b'b' || s[2] != b'c' || s[3] != b'\0' {
        return 1;
    }

    if t[0] != b'a' || t[1] != b'b' || t[2] != b'c' {
        return 2;
    }

    s[0] = b'x';
    let mut t = t.clone();
    t[2] = b'y';

    if s[0] != b'x' {
        return 3;
    }
    if t[2] != b'y' {
        return 4;
    }

    {
        let p = "abc";
        if p.as_bytes()[0] != b'a' || p.as_bytes()[1] != b'b' || p.as_bytes()[2] != b'c' || p.as_bytes()[3] != b'\0' {
            return 5;
        }
    }

    0
}