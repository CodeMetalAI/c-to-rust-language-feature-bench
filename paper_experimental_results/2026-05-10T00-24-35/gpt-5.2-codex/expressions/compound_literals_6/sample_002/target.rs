use std::process::exit;

fn main() {
    let a: [u8; 4] = *b"abc\0";
    let b: [u8; 4] = *b"abc\0";

    if a[0] != b'a' || a[1] != b'b' || a[2] != b'c' || a[3] != 0 {
        exit(1);
    }
    if b[0] != b'a' || b[1] != b'b' || b[2] != b'c' || b[3] != 0 {
        exit(2);
    }

    let a_eq = std::ptr::eq(a.as_ptr(), b.as_ptr());
    let a_eq_int = if a_eq { 1 } else { 0 };
    if a_eq_int != 0 && a_eq_int != 1 {
        exit(3);
    }

    exit(0);
}