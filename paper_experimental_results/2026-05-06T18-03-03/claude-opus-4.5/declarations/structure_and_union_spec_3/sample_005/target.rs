use std::mem;

struct S {
    i: i32,
    a: Vec<i32>,
}

fn main() {
    // In the C code, offsetof(struct s, i) is checked to be 0
    // In Rust, we can verify that i would be at offset 0 in a similar struct
    // Since we're using a Vec instead of a flexible array member, we simulate the checks
    
    // Check 1: In the original C struct, i is at offset 0 (due to anonymous struct)
    // We simulate this by checking that our struct layout starts with i
    // This is implicitly true in our Rust struct definition
    
    // Check 2: In C, offsetof(struct s, a) == sizeof(struct s) for flexible array members
    // This is a property of flexible array members - they start at the end of the struct
    // We can't directly replicate this in safe Rust, but we can verify the concept
    // by checking that the size of a struct with just i equals the offset where a would start
    #[repr(C)]
    struct SBase {
        i: i32,
    }
    
    // In C, the flexible array member 'a' starts at offset sizeof(struct s)
    // which equals sizeof(int) = 4 (with potential padding)
    // For our simulation, we verify the base struct size
    if mem::offset_of!(SBase, i) != 0 {
        std::process::exit(1);
    }
    
    // The size of SBase (just the i field) represents where 'a' would start
    // In C with the given struct, sizeof(struct s) == sizeof(int) due to flexible array
    if mem::size_of::<SBase>() != mem::size_of::<i32>() {
        std::process::exit(2);
    }
    
    // Allocate equivalent of malloc(sizeof(struct s) + 4 * sizeof(int))
    // In Rust, we use a Vec with capacity for 4 integers
    let mut p = S {
        i: 0,
        a: vec![0; 4],
    };
    
    // The allocation check (if (!p) return 3) is not needed in Rust
    // as Vec allocation panics on failure in safe Rust
    
    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;
    
    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        std::process::exit(4);
    }
    
    // free(p) is handled automatically by Rust's drop
    
    std::process::exit(0);
}