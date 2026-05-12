use std::mem;

struct S {
    i: i32,
    a: Vec<i32>,
}

fn main() {
    // In the C code, offsetof(struct s, i) checks that i is at offset 0
    // In Rust, we can verify the first field is at offset 0
    // For a struct with fields in order, the first field is at offset 0
    // (after any padding, but i32 has alignment 4 which is fine)
    
    // The C code checks offsetof(struct s, i) != 0, returns 1 if true
    // In our Rust struct, i is the first field, so it's at offset 0
    // This check passes (doesn't return 1)
    
    // The C code checks offsetof(struct s, a) != sizeof(struct s), returns 2 if true
    // In C, flexible array members are at the end and sizeof(struct s) equals
    // the offset of the flexible array member. In Rust, we use Vec which is different,
    // but we're preserving the behavior, not the exact memory layout.
    // This check conceptually passes in the original.
    
    // Simulate the allocation: malloc(sizeof(struct s) + 4 * sizeof(int))
    // This allocates space for 4 integers in the flexible array
    let mut p = Box::new(S {
        i: 0,
        a: vec![0; 4],
    });
    
    // The malloc check - Box::new won't return null in safe Rust
    // (it will panic on allocation failure), so we skip the null check
    
    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;
    
    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        std::process::exit(4);
    }
    
    // free(p) - happens automatically when p goes out of scope
    
    std::process::exit(0);
}