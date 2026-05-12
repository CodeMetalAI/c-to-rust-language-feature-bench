use std::process::exit;

fn main() {
    let a_arr: [u8; 4] = *b"abc\0";
    let b_arr: &[u8; 4] = b"abc\0";

    let a = &a_arr;
    let b = b_arr;

    if a[0] != b'a' || a[1] != b'b' || a[2] != b'c' || a[3] != b'\0' {
        exit(1);
    }
    if b[0] != b'a' || b[1] != b'b' || b[2] != b'c' || b[3] != b'\0' {
        exit(2);
    }

    let eq = a.as_ptr() == b.as_ptr();
    if eq != false && eq != true {
        exit(3);
    }

    exit(0);
}