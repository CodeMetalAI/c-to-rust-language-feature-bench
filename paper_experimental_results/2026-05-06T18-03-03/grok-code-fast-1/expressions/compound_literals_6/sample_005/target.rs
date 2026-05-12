use std::ptr;

fn main() {
    let temp: [u8; 4] = [b'a', b'b', b'c', 0];
    let a: &[u8] = &temp;
    let b: &[u8] = b"abc\0";

    if a[0] != b'a' || a[1] != b'b' || a[2] != b'c' || a[3] != 0 {
        std::process::exit(1);
    }
    if b[0] != b'a' || b[1] != b'b' || b[2] != b'c' || b[3] != 0 {
        std::process::exit(2);
    }

    let eq = ptr::eq(a.as_ptr(), b.as_ptr()) as i32;
    if eq != 0 && eq != 1 {
        std::process::exit(3);
    }

    std::process::exit(0);
}