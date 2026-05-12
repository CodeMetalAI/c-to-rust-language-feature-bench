fn main() {
    let mut a = [10, 20, 30];

    // In Rust, `a` is an array and `&a` is a reference to an array
    if !matches!(a, [_, _, _]) {
        std::process::exit(1);
    }
    if !matches!(&a, &[_; 3]) {
        std::process::exit(2);
    }

    // Direct access to array elements
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    // Size and alignment checks
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s = "abc";
    // In Rust, string literals are &str, automatically null-terminated
    if !matches!(s, "abc") {
        std::process::exit(8);
    }
    let s_bytes = s.as_bytes();
    if s_bytes[0] != b'a' {
        std::process::exit(9);
    }
    if s_bytes[1] != b'b' {
        std::process::exit(10);
    }
    if s_bytes[2] != b'c' {
        std::process::exit(11);
    }
    if s_bytes.len() != 3 { // Check for null termination in the original C array
        std::process::exit(12);
    }
}