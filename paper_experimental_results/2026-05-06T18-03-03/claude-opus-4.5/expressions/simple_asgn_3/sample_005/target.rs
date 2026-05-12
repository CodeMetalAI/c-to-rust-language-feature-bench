fn main() {
    let mut c: u8 = b'A';
    
    // In the original C code, through pointer aliasing tricks,
    // the const char c gets modified to 0.
    // We simulate this by directly modifying c.
    c = 0;
    
    if c != b'A' {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}