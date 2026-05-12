static B_ARR: [u8; 4] = *b"abc\0";

fn main() {
    let a_arr: [u8; 4] = *b"abc\0";

    if a_arr[0] != b'a' || a_arr[1] != b'b' || a_arr[2] != b'c' || a_arr[3] != b'\0' {
        std::process::exit(1);
    }
    if B_ARR[0] != b'a' || B_ARR[1] != b'b' || B_ARR[2] != b'c' || B_ARR[3] != b'\0' {
        std::process::exit(2);
    }

    let a_ptr = a_arr.as_ptr();
    let b_ptr = B_ARR.as_ptr();
    let eq = (a_ptr == b_ptr) as i32;

    if (eq != 0) && (eq != 1) {
        std::process::exit(3);
    }

    std::process::exit(0);
}