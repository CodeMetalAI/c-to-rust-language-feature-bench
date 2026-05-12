fn main() {
    let a: [u8; 4] = *b"abc\0";
    let b: [u8; 4] = *b"abc\0";

    if a[0] != b'a' || a[1] != b'b' || a[2] != b'c' || a[3] != 0 {
        std::process::exit(1);
    }
    if b[0] != b'a' || b[1] != b'b' || b[2] != b'c' || b[3] != 0 {
        std::process::exit(2);
    }

    let cmp = std::ptr::eq(a.as_ptr(), b.as_ptr());
    let cmp_i = if cmp { 1 } else { 0 };
    if cmp_i != 0 && cmp_i != 1 {
        std::process::exit(3);
    }
}