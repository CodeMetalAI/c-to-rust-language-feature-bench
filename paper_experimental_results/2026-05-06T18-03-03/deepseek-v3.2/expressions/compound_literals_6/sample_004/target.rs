fn main() {
    let a = "abc".as_ptr();
    let b = "abc".as_ptr();

    let a_slice = unsafe { std::slice::from_raw_parts(a, 4) };
    let b_slice = unsafe { std::slice::from_raw_parts(b, 4) };

    if a_slice[0] != b'a' || a_slice[1] != b'b' || a_slice[2] != b'c' || a_slice[3] != b'\0' {
        std::process::exit(1);
    }
    if b_slice[0] != b'a' || b_slice[1] != b'b' || b_slice[2] != b'c' || b_slice[3] != b'\0' {
        std::process::exit(2);
    }

    let equal = a == b;
    if equal != true && equal != false {
        std::process::exit(3);
    }

    std::process::exit(0);
}