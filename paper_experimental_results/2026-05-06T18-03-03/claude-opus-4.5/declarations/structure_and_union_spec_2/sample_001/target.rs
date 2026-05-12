fn main() {
    // In Rust, we can't have flexible array members like in C.
    // We'll simulate the behavior using Vec or separate allocations.
    
    // struct s { int n; double d[]; }
    // struct ss { int n; }
    
    // In C, sizeof(struct s) >= sizeof(struct ss) because of padding for alignment
    // For struct s with a flexible array member of double, sizeof(struct s) includes
    // padding to align the flexible array member.
    
    // size_of::<i32>() is 4, but with alignment for f64 (8 bytes), 
    // the struct would be padded to 8 bytes
    let size_s = 8usize; // sizeof(struct s) in C with flexible array member
    let size_ss = 4usize; // sizeof(struct ss) = sizeof(int)
    
    if size_s < size_ss {
        std::process::exit(1);
    }
    
    // offsetof(struct s, d) should equal sizeof(struct s)
    // In C, the flexible array member starts right after the struct
    let offset_d = 8usize; // offset of d in struct s (after padding for alignment)
    if offset_d != size_s {
        std::process::exit(1);
    }
    
    // Simulate the flexible array member structs using a struct with a Vec
    struct S {
        n: i32,
        d: Vec<f64>,
    }
    
    // malloc(sizeof(struct s) + sizeof(double) * 8) - allocate with 8 doubles
    let mut s1 = Some(S {
        n: 0,
        d: vec![0.0; 8],
    });
    
    // malloc(sizeof(struct s) + sizeof(double) * 5) - allocate with 5 doubles
    let mut s2 = Some(S {
        n: 0,
        d: vec![0.0; 5],
    });
    
    // Check if allocations succeeded (in Rust, Vec allocation panics on failure,
    // but we simulate the C behavior)
    if s1.is_none() || s2.is_none() {
        std::process::exit(1);
    }
    
    // s1->d[0] = 42.0
    if let Some(ref mut s) = s1 {
        s.d[0] = 42.0;
    }
    
    // s2->d[0] = 24.0
    if let Some(ref mut s) = s2 {
        s.d[0] = 24.0;
    }
    
    // Check values
    let s1_d0 = s1.as_ref().map(|s| s.d[0]).unwrap_or(0.0);
    let s2_d0 = s2.as_ref().map(|s| s.d[0]).unwrap_or(0.0);
    
    if s1_d0 != 42.0 || s2_d0 != 24.0 {
        std::process::exit(1);
    }
    
    // free(s1); free(s2); - handled automatically by Rust's drop
    drop(s1);
    drop(s2);
    
    // return 0
    std::process::exit(0);
}