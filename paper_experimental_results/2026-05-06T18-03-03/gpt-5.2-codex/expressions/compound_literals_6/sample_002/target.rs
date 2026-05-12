fn main() {
    let a_array: [u8; 4] = *b"abc\0";
    let b_array: [u8; 4] = *b"abc\0";

    let a_ptr = a_array.as_ptr();
    let b_ptr = b_array.as_ptr();

    if a_array[0] != b'a' || a_array[1] != b'b' || a_array[2] != b'c' || a_array[3] != b'\0' {
        std::process::exit(1);
    }
    if b_array[0] != b'a' || b_array[1] != b'b' || b_array[2] != b'c' || b_array[3] != b'\0' {
        std::process::exit(2);
    }

    let eq = (a_ptr == b_ptr) as i32;
    if eq != 0 && eq != 1 {
        std::process::exit(3);
    }

    std::process::exit(0);
}