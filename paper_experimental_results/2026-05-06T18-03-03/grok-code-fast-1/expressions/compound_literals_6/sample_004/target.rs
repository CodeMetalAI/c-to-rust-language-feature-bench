fn main() {
    let a: &[u8] = &[b'a', b'b', b'c', b'\0'];
    let b: &[u8] = b"abc\0";

    if a[0] != b'a' || a[1] != b'b' || a[2] != b'c' || a[3] != b'\0' {
        std::process::exit(1);
    }
    if b[0] != b'a' || b[1] != b'b' || b[2] != b'c' || b[3] != b'\0' {
        std::process::exit(2);
    }

    // The condition (a == b) != 0 && (a == b) != 1 is always false in C++,
    // so we skip it and return 0.

    std::process::exit(0);
}