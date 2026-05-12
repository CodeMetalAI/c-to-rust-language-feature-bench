fn main() -> i32 {
    let a: &[u8] = b"abc\0";
    let b: &[u8] = b"abc\0";

    if a[0] != b'a' || a[1] != b'b' || a[2] != b'c' || a[3] != b'\0' {
        return 1;
    }
    if b[0] != b'a' || b[1] != b'b' || b[2] != b'c' || b[3] != b'\0' {
        return 2;
    }

    let eq = a == b;
    if (eq as i32 != 0) && (eq as i32 != 1) {
        return 3;
    }

    0
}