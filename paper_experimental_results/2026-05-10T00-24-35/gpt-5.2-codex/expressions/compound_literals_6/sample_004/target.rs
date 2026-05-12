use std::process::exit;

fn main() {
    let a_arr = *b"abc\0";
    let b_arr: &'static [u8; 4] = b"abc\0";

    if a_arr[0] != b'a' || a_arr[1] != b'b' || a_arr[2] != b'c' || a_arr[3] != 0 {
        exit(1);
    }
    if b_arr[0] != b'a' || b_arr[1] != b'b' || b_arr[2] != b'c' || b_arr[3] != 0 {
        exit(2);
    }

    let a_ptr = a_arr.as_ptr();
    let b_ptr = b_arr.as_ptr();
    let ptr_eq = std::ptr::eq(a_ptr, b_ptr);

    if (ptr_eq != false) && (ptr_eq != true) {
        exit(3);
    }

    exit(0);
}