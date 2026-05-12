fn main() {
    let a = ["a" as u8, 'b' as u8, 'c' as u8, 0];
    let b = "abc";

    if a[0] != b'a' || a[1] != b'b' || a[2] != b'c' || a[3] != 0 {
        return 1;
    }
    if b.as_bytes()[0] != b'a' || b.as_bytes()[1] != b'b' || b.as_bytes()[2] != b'c' || b.as_bytes()[3] != 0 {
        return 2;
    }

    if (a.as_ptr() as usize == b.as_ptr() as usize) != 0 && (a.as_ptr() as usize == b.as_ptr() as usize) != 1 {
        return 3;
    }

    return 0;
}