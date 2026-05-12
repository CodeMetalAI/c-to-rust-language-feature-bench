fn main() {
    let a: &[u8] = b"abc";
    let b: &[u8] = b"abc";

    if a[0] != b'a' || a[1] != b'b' || a[2] != b'c' || a.len() != 3 {
        std::process::exit(1);
    }
    if b[0] != b'a' || b[1] != b'b' || b[2] != b'c' || b.len() != 3 {
        std::process::exit(2);
    }

    let a_ptr = a.as_ptr();
    let b_ptr = b.as_ptr();
    let equal = a_ptr == b_ptr;
    if !(equal == false || equal == true) {
        std::process::exit(3);
    }

    std::process::exit(0);
}