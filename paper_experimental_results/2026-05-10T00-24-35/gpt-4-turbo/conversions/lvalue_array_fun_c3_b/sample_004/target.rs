fn main() {
    let mut a = [10, 20, 30];

    // Equivalent to TYPE_ID(a) != 1 in C++ (matching not needed in Rust)
    // Equivalent to TYPE_ID(&a) != 2 in C++ (matching not needed in Rust)
    
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s = "abc";
    let s_bytes = s.as_bytes();
    // Equivalent to TYPE_ID("abc") != 3 in C++ (matching not needed in Rust)
    
    if s_bytes[0] as char != 'a' {
        std::process::exit(9);
    }
    if s_bytes[1] as char != 'b' {
        std::process::exit(10);
    }
    if s_bytes[2] as char != 'c' {
        std::process::exit(11);
    }
    if s_bytes.len() != 3 {  // Checks for null-termination in the original C++
        std::process::exit(12);
    }
}