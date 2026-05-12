fn main() {
    let c: char = 'A';
    
    // In the original C code, this is undefined behavior.
    // The const 'c' is declared as 'A', and the compiler is allowed
    // to assume it stays 'A'. With optimizations, c != 'A' is false.
    // We replicate the observable behavior: the program returns 0.
    
    if c != 'A' {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}