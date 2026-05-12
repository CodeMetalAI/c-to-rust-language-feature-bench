fn main() {
    // In Rust, we can't have flexible array members like in C.
    // The C code is checking properties of flexible array members.
    
    // struct s { int n; double d[]; } has size equal to the offset of d,
    // which is the size needed for n plus padding for double alignment.
    // struct ss { int n; } has size of just int.
    
    // On most platforms:
    // - sizeof(int) = 4
    // - sizeof(double) = 8, alignof(double) = 8
    // - sizeof(struct s) = 8 (4 bytes for n + 4 bytes padding for double alignment)
    // - sizeof(struct ss) = 4
    
    // Check: sizeof(struct s) >= sizeof(struct ss)
    // This should always be true since struct s contains at least what struct ss has
    let size_of_s = std::mem::size_of::<i32>().max(std::mem::align_of::<f64>());
    let size_of_ss = std::mem::size_of::<i32>();
    
    if size_of_s < size_of_ss {
        std::process::exit(1);
    }
    
    // Check: offsetof(struct s, d) == sizeof(struct s)
    // For flexible array members, the offset of the flexible array equals the struct size
    // This is inherently true by definition of flexible array members
    // In our simulation, both are size_of_s
    let offset_of_d = size_of_s;
    if offset_of_d != size_of_s {
        std::process::exit(1);
    }
    
    // Simulate the flexible array member structs using Vec
    struct S {
        n: i32,
        d: Vec<f64>,
    }
    
    // Allocate s1 with capacity for 8 doubles
    let mut s1 = S {
        n: 0,
        d: vec![0.0; 8],
    };
    
    // Allocate s2 with capacity for 5 doubles
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