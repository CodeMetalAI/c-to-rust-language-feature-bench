fn main() {
    let p = "x ## y";
    let bytes = p.as_bytes();

    if bytes.len() != 6 {
        std::process::exit(1);
    }
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
    // The C string includes a null terminator, but Rust strings don't.
    // The original check was for size 7 including null terminator.
    // We'll check that the string length is 6 (without null) and that
    // the last character is 'y' (already done above).
    // No need to check for null terminator separately.
    
    std::process::exit(0);
}