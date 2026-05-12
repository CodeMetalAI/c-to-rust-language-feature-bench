fn main() {
    let mut a = [10, 20, 30];

    // In Rust, `a` is always of type `[i32; 3]` and `&a` is `&[i32; 3]`
    // There's no direct equivalent to C's _Generic, but we can use pattern matching
    // or type checking functions if needed. Here, we just use the type directly.

    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    // `std::mem::size_of_val` gives the size in bytes of the value it's called on
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    // `std::mem::align_of_val` gives the alignment of the type of the value
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s = "abc";
    // In Rust, string literals are always `&str` and not mutable
    if s.as_bytes()[0] != b'a' {
        std::process::exit(9);
    }
    if s.as_bytes()[1] != b'b' {
        std::process::exit(10);
    }
    if s.as_bytes()[2] != b'c' {
        std::process::exit(11);
    }
    if s.as_bytes().get(3) != Some(&b'\0') {
        std::process::exit(12);
    }
}