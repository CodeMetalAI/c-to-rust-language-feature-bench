use std::process::exit;

static B: [u8; 4] = *b"abc\0";

fn main() {
    let a: [u8; 4] = *b"abc\0";

    if a[0] != b'a' || a[1] != b'b' || a[2] != b'c' || a[3] != 0 {
        exit(1);
    }
    if B[0] != b'a' || B[1] != b'b' || B[2] != b'c' || B[3] != 0 {
        exit(2);
    }

    let eq = std::ptr::eq(a.as_ptr(), B.as_ptr());
    let eq_i = if eq { 1 } else { 0 };
    if eq_i != 0 && eq_i != 1 {
        exit(3);
    }

    exit(0);
}