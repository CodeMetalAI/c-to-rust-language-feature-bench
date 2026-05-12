fn main() {
    let c: char = 'A';
    
    // In safe Rust, we cannot replicate the undefined behavior of the C code
    // which attempts to modify a const through pointer aliasing.
    // The C code has undefined behavior, but typically returns 0
    // because compilers optimize assuming const values don't change.
    
    if c != 'A' {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}