fn main() {
    let p = "x ## y";
    let p_bytes = p.as_bytes();
    
    if p_bytes.len() != 6 {
        std::process::exit(1);
    }
    if p_bytes[0] != b'x' {
        std::process::exit(2);
    }
    if p_bytes[1] != b' ' {
        std::process::exit(3);
    }
    if p_bytes[2] != b'#' {
        std::process::exit(4);
    }
    if p_bytes[3] != b'#' {
        std::process::exit(5);
    }
    if p_bytes[4] != b' ' {
        std::process::exit(6);
    }
    if p_bytes[5] != b'y' {
        std::process::exit(7);
    }
    // C string includes null terminator in sizeof, Rust strings don't
    // The original check was for 7 bytes including null terminator
    // So we need to check the actual string length (without null) is 6
    if p.len() != 6 {
        std::process::exit(8);
    }
    
    std::process::exit(0);
}