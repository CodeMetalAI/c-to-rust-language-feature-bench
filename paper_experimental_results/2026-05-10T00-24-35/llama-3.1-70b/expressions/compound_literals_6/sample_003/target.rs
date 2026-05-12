fn main() {
    let a = [b'a', b'b', b'c', b'\0'];
    let b = [b'a', b'b', b'c', b'\0'];

    if a[0] != b'a' || a[1] != b'b' || a[2] != b'c' || a[3] != b'\0' {
        return 1;
    }
    if b[0] != b'a' || b[1] != b'b' || b[2] != b'c' || b[3] != b'\0' {
        return 2;
    }

    if (a.as_ptr() == b.as_ptr()) as i32 != 0 && (a.as_ptr() == b.as_ptr()) as i32 != 1 {
        return 3;
    }

    return 0;
}