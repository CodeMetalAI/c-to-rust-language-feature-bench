fn main() {
    let a = [b'a', b'b', b'c'];
    let b = "abc";

    if a[0] != b'a' || a[1] != b'b' || a[2] != b'c' || a.len() != 3 {
        return 1;
    }
    if b[0] != b'a' || b[1] != b'b' || b[2] != b'c' || b.len() != 3 {
        return 2;
    }

    if &a as *const [u8] == &b as *const [u8] {
        return 3;
    }

    return 0;
}