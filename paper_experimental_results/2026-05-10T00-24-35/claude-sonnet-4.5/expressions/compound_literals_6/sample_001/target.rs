fn main() {
    let a: &[u8] = b"abc";
    let b: &[u8] = b"abc";

    if a[0] != b'a' || a[1] != b'b' || a[2] != b'c' || a[3] != b'\0' {
        std::process::exit(1);
    }
    if b[0] != b'a' || b[1] != b'b' || b[2] != b'c' || b[3] != b'\0' {
        std::process::exit(2);
    }

    let cmp = if a.as_ptr() == b.as_ptr() { 1 } else { 0 };
    if cmp != 0 && cmp != 1 {
        std::process::exit(3);
    }

    std::process::exit(0);
}