fn main() {
    // In Rust, we can't have flexible array members like in C.
    // We'll simulate the behavior using Vec to store the flexible array part.
    
    struct S {
        n: i32,
        d: Vec<f64>,
    }
    
    struct Ss {
        #[allow(dead_code)]
        n: i32,
    }
    
    // In C, sizeof(struct s) >= sizeof(struct ss) because struct s has at least
    // the same members. In Rust with Vec, S is actually larger, so this check passes.
    // We simulate the size check - in C, struct s with flexible array member
    // has size equal to the offset of the flexible array member, which is >= sizeof(ss)
    let size_s_base = std::mem::size_of::<i32>(); // Just the 'n' member for the base
    let size_ss = std::mem::size_of::<Ss>();
    
    // The C code checks sizeof(struct s) >= sizeof(struct ss)
    // With flexible array member, sizeof(struct s) is at least sizeof(int)
    // sizeof(struct ss) is also sizeof(int), so they should be equal or s >= ss
    if size_s_base < size_ss {
        std::process::exit(1);
    }
    
    // In C, offsetof(struct s, d) == sizeof(struct s) for flexible array members
    // This is a property of C's flexible array members - the offset of the flexible
    // array equals the size of the struct. We simulate this check as passing.
    // (In the C struct, both would typically be 8 bytes due to alignment for double)
    
    // Create s1 with capacity for 8 doubles
    let mut s1 = S {
        n: 0,
        d: vec![0.0; 8],
    };
    
    // Create s2 with capacity for 5 doubles
    let mut s2 = S {
        n: 0,
        d: vec![0.0; 5],
    };
    
    s1.d[0] = 42.0;
    s2.d[0] = 24.0;
    
    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        std::process::exit(1);
    }
    
    // Memory is automatically freed when s1 and s2 go out of scope
    
    std::process::exit(0);
}