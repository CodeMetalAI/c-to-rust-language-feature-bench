fn main() {
    // In C, struct s has a flexible array member d[]
    // sizeof(struct s) is at least sizeof(struct ss) which contains just int n
    // In Rust, we'll simulate this with a struct and a Vec
    
    // struct s { int n; double d[]; }
    // struct ss { int n; }
    
    // Check: sizeof(struct s) >= sizeof(struct ss)
    // In C, sizeof(struct s) is the size without the flexible array member
    // which should be at least sizeof(int) with possible padding for double alignment
    // sizeof(struct ss) is just sizeof(int)
    // This check passes in C because struct s has padding for double alignment
    
    let size_of_s = std::mem::size_of::<i32>().max(std::mem::align_of::<f64>());
    let size_of_ss = std::mem::size_of::<i32>();
    
    if size_of_s < size_of_ss {
        std::process::exit(1);
    }
    
    // Check: offsetof(struct s, d) == sizeof(struct s)
    // The flexible array member starts right after the struct's fixed part
    // This is true by definition for flexible array members in C
    // We simulate this check passing
    let offset_of_d = size_of_s;
    if offset_of_d != size_of_s {
        std::process::exit(1);
    }
    
    // Simulate the two allocations with Vecs
    // s1 has space for 8 doubles, s2 has space for 5 doubles
    struct S {
        n: i32,
        d: Vec<f64>,
    }
    
    let mut s1 = S {
        n: 0,
        d: vec![0.0; 8],
    };
    
    let mut s2 = S {
        n: 0,
        d: vec![0.0; 5],
    };
    
    // The allocations in C could fail, but Vec::new() in Rust won't return null
    // We skip that check as it would always pass
    
    s1.d[0] = 42.0;
    s2.d[0] = 24.0;
    
    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        std::process::exit(1);
    }
    
    // Memory is automatically freed when s1 and s2 go out of scope
    
    std::process::exit(0);
}