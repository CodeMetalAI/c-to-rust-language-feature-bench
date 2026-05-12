fn main() {
    let c: char = 'A';
    
    // In safe Rust, we cannot create the aliasing situation that the C code attempts.
    // The C code has undefined behavior - it tries to modify a const through pointer tricks.
    // In Rust, immutable bindings truly cannot be modified through safe code.
    // The check will always pass since c cannot be mutated.
    
    if c != 'A' {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}