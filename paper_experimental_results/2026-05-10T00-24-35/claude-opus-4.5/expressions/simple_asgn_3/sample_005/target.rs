fn main() {
    let c: char = 'A';
    
    // In safe Rust, we cannot perform the aliasing/const-violating
    // pointer manipulation from the C code.
    // The C code has undefined behavior; in practice on most systems,
    // either c remains 'A' (compiler optimizes based on const),
    // or the program crashes (if c is in read-only memory).
    // We preserve the "well-defined" path where c remains unchanged.
    
    if c != 'A' {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}