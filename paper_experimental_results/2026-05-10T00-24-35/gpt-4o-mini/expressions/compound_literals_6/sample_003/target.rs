fn main() {
    let a = "abc".as_bytes();
    let b = "abc".as_bytes();

    if a[0] != b'a' || a[1] != b'b' || a[2] != b'c' || a[3] != b'\0' {
        std::process::exit(1);
    }
    if b[0] != b'a' || b[1] != b'b' || b[2] != b'c' || b[3] != b'\0' {
        std::process::exit(2);
    }

    if (a.as_ptr() == b.as_ptr()) as i32 != 0 && (a.as_ptr() == b.as_ptr()) as i32 != 1 {
        std::process::exit(3);
    }

    std::process::exit(0);
}