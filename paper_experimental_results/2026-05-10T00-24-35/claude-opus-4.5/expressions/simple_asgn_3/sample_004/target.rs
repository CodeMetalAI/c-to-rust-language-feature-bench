fn main() {
    let c: char = 'A';
    
    // In safe Rust, we cannot actually perform the aliasing trick
    // that the C code attempts. The C code has undefined behavior.
    // A typical C compiler will either:
    // 1. Optimize away the modification (c stays 'A', return 0)
    // 2. Actually modify c (c becomes 0, return 1)
    // 
    // Most compilers with optimization will keep c as 'A' due to
    // const propagation, so we replicate that behavior.
    
    if c != 'A' {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}