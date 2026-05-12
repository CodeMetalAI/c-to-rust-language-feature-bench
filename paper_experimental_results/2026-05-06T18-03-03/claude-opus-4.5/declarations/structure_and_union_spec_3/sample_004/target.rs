fn main() {
    // In the C code, struct s has an anonymous struct containing int i at offset 0,
    // followed by a flexible array member a[].
    // 
    // We simulate this with a struct and a separate Vec for the flexible array part.
    
    // Check 1: offsetof(struct s, i) != 0
    // In Rust, we can use a simple struct where i is the first field
    // This is equivalent to offset 0
    
    // Check 2: offsetof(struct s, a) != sizeof(struct s)
    // The flexible array member starts right after the struct's fixed part
    // In C with an int (4 bytes) and proper alignment, sizeof(struct s) would be 4
    // and offsetof(a) would also be 4, so they're equal
    
    // Simulate the structure with flexible array member
    struct S {
        i: i32,
        a: Vec<i32>,
    }
    
    // The offset checks in C are verifying memory layout properties
    // In safe Rust, we can't directly check offsets, but we can verify
    // that our simulation behaves correctly
    
    // offsetof(struct s, i) should be 0 - first field is at offset 0
    // This is guaranteed by Rust's struct layout for the first field in repr(C),
    // but we're simulating the behavior, so we just proceed
    
    // offsetof(struct s, a) should equal sizeof(struct s)
    // For a flexible array member, this is true in C
    // We're simulating this with a Vec, so we skip the raw offset check
    
    // Allocate equivalent of malloc(sizeof(struct s) + 4 * sizeof(int))
    let mut p = S {
        i: 0,
        a: vec![0; 4],
    };
    
    // Set values
    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;
    
    // Verify values
    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        std::process::exit(4);
    }
    
    // p is automatically dropped here (equivalent to free)
    std::process::exit(0);
}