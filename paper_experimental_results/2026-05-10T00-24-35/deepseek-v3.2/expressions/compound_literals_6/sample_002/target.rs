fn main() {
    let a: &'static [u8] = b"abc";
    let b: &'static str = "abc";

    if a[0] != b'a' || a[1] != b'b' || a[2] != b'c' || a[3] != b'\0' {
        std::process::exit(1);
    }
    let b_bytes = b.as_bytes();
    if b_bytes[0] != b'a' || b_bytes[1] != b'b' || b_bytes[2] != b'c' || b_bytes[3] != b'\0' {
        std::process::exit(2);
    }

    let a_ptr = a as *const _;
    let b_ptr = b_bytes as *const _;
    let equal = a_ptr == b_ptr;
    if equal != false && equal != true {
        std::process::exit(3);
    }

    std::process::exit(0);
}