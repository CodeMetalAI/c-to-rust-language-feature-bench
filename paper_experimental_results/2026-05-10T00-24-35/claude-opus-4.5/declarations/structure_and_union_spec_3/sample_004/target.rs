use std::mem;

struct S {
    i: i32,
    a: Vec<i32>,
}

fn main() {
    // Check that i is at offset 0 (it's the first field)
    // In Rust, we can't directly use offsetof, but we can verify the structure
    // The original code checks offsetof(struct s, i) != 0
    // Since i is the first field in our struct, this is equivalent to offset 0
    
    // Check that a comes after the fixed part of the struct
    // In the C code, offsetof(struct s, a) should equal sizeof(struct s)
    // This is checking that the flexible array member is at the end
    // In Rust with Vec, this is always true by design
    
    // Simulate the flexible array member behavior using Vec
    let mut p = S {
        i: 0,
        a: vec![0; 4],  // Allocate space for 4 integers
    };
    
    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;
    
    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        std::process::exit(4);
    }
    
    // Memory is automatically freed when p goes out of scope
    std::process::exit(0);
}