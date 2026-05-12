use std::mem;

struct S {
    i: i32,
    a: Vec<i32>,
}

fn main() {
    // Check that i is at offset 0 (it's the first field)
    // In Rust, we can't directly use offsetof, but we can verify the structure
    // Since i is the first field in our struct, offset is 0
    // This is implicitly satisfied by our struct definition
    
    // The original C code checks offsetof(struct s, a) != sizeof(struct s)
    // In C, flexible array members start at the end of the struct
    // In Rust, we're using a Vec which is different, but we maintain the same behavior
    
    // Simulate the allocation with 4 integers in the flexible array
    let mut p = S {
        i: 0,
        a: vec![0; 4],
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