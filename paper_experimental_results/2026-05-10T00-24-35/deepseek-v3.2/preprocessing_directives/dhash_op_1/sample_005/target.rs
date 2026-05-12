fn main() {
    let p = "x ## y";
    
    if p.len() != 6 {
        std::process::exit(1);
    }
    
    let bytes = p.as_bytes();
    
    if bytes[0] != b'x' {
        std::process::exit(2);
    }
    if bytes[1] != b' ' {
        std::process::exit(3);
    }
    if bytes[2] != b'#' {
        std::process::exit(4);
    }
    if bytes[3] != b'#' {
        std::process::exit(5);
    }
    if bytes[4] != b' ' {
        std::process::exit(6);
    }
    if bytes[5] != b'y' {
        std::process::exit(7);
    }
    
    // In C, the string "x ## y" has length 7 including the null terminator
    // In Rust, strings don't include null terminators, so we need to check the actual length
    // The C code expects sizeof(p) = 7, which includes 6 characters + null terminator
    // Rust's len() returns 6 for "x ## y", which matches the number of characters
    // So we keep the check as p.len() != 6 (not 7)
    
    // Note: The original C code checks p[6] != '\0', which we don't need in Rust
    // since Rust strings aren't null-terminated. But the original check for
    // sizeof(p) != 7 includes the null terminator, so we match that with len() != 6
    
    std::process::exit(0);
}