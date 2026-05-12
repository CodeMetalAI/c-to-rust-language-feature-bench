use std::process::exit;

fn main() {
    let a_arr: [u8; 4] = *b"abc\0";
    const B: &[u8; 4] = b"abc\0";
    let b_arr = B;

    if a_arr[0] != b'a' || a_arr[1] != b'b' || a_arr[2] != b'c' || a_arr[3] != b'\0' {
        exit(1);
    }
    if b_arr[0] != b'a' || b_arr[1] != b'b' || b_arr[2] != b'c' || b_arr[3] != b'\0' {
        exit(2);
    }

    let a_ptr = a_arr.as_ptr();
    let b_ptr = b_arr.as_ptr();
    let eq = if a_ptr == b_ptr { 1 } else { 0 };
    if eq != 0 && eq != 1 {
        exit(3);
    }
}