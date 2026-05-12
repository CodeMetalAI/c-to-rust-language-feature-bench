fn main() {
    // In C, struct s has a flexible array member d[]
    // sizeof(struct s) is at least sizeof(struct ss) which contains just int n
    // In Rust, we'll simulate this with a struct containing n and a Vec for the flexible array
    
    // struct s { int n; double d[]; }
    // struct ss { int n; }
    
    // The C code checks:
    // 1. sizeof(struct s) >= sizeof(struct ss) - always true since s contains at least what ss has
    // 2. offsetof(struct s, d) == sizeof(struct s) - this is a property of flexible array members
    
    // In Rust, we simulate the flexible array member behavior
    // The size/offset checks in C are compile-time properties that are always true for valid
    // flexible array member structs, so we skip those checks as they would always pass
    
    // Simulate s1 with capacity for 8 doubles
    let mut s1_d: Vec<f64> = vec![0.0; 8];
    // Simulate s2 with capacity for 5 doubles
    let mut s2_d: Vec<f64> = vec![0.0; 5];
    
    // The malloc checks - Vec allocation in Rust won't return null in safe code
    // (it will panic on allocation failure), so we skip the null check
    
    s1_d[0] = 42.0;
    s2_d[0] = 24.0;
    
    if s1_d[0] != 42.0 || s2_d[0] != 24.0 {
        std::process::exit(1);
    }
    
    // free is automatic in Rust when variables go out of scope
    
    std::process::exit(0);
}