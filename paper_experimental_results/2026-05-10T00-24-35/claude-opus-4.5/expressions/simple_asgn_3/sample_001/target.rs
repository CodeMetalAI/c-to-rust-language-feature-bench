fn main() {
    let c: char = 'A';
    
    // The original C code has undefined behavior due to modifying a const
    // through pointer aliasing. In safe Rust, we cannot replicate this UB.
    // The observable behavior depends on compiler optimizations.
    // With const optimization (common), c remains 'A', so we return 0.
    
    if c != 'A' {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}