fn main() {
    // Equivalent Rust code for the C array and type checks
    let mut a = [10, 20, 30];

    // In Rust, we don't have a direct analog to C's _Generic keyword or sizeof operator
    // We use standard functions and methods to achieve the desired checks

    // Checking the content of the array
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    // Using std::mem::size_of to check the size of the array
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }

    // Rust doesn't expose alignment check directly in the same way as C,
    // but we can use std::mem::align_of to get the alignment.
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    // Handling the string "abc"
    let s = "abc";
    let s_bytes = s.as_bytes();

    // Rust strings are UTF-8 and automatically null-terminated when needed for C FFI but not stored with the null in Rust
    if s_bytes[0] != b'a' {
        std::process::exit(9);
    }
    if s_bytes[1] != b'b' {
        std::process::exit(10);
    }
    if s_bytes[2] != b'c' {
        std::process::exit(11);
    }
    if s.len() != 3 { // Rust strings do not include a null terminator in their length
        std::process::exit(12);
    }
}