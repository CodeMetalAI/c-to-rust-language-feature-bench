fn main() {
    let c: char = 'A';
    
    // In the original C code, this is undefined behavior.
    // The const 'c' is declared but then modified through pointer tricks.
    // Most compilers with optimization will still see c as 'A' due to const propagation.
    // In safe Rust, we cannot replicate this UB, so we preserve the expected behavior.
    
    if c != 'A' {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}