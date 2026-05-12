fn main() {
    // In the original C code, through pointer aliasing tricks,
    // the "const" char c gets modified from 'A' to 0.
    // Since we can't do that aliasing in safe Rust, we simulate
    // the actual runtime behavior: c becomes 0, so c != 'A' is true,
    // and the program returns exit code 1.
    
    let mut c: u8 = b'A';
    
    // Simulating the effect of the pointer aliasing: *p = 0 modifies c
    c = 0;
    
    if c != b'A' {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}