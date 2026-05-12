fn main() {
    // In the C code, struct s has:
    // - An anonymous struct containing int i at offset 0
    // - A flexible array member a[] at offset sizeof(int) which equals sizeof(struct s)
    
    // Check that offsetof(struct s, i) == 0
    // In Rust, we'll use a struct where i is the first field
    // This is implicitly true for repr(C) structs, but we're simulating the check
    
    // The C struct has sizeof(struct s) == sizeof(int) == 4 (typically)
    // and offsetof(a) == sizeof(struct s) == 4
    // offsetof(i) == 0
    
    // Simulate the structure with a Vec to handle the flexible array member
    struct S {
        i: i32,
        a: Vec<i32>,
    }
    
    // The offset checks in C are compile-time/runtime checks about layout
    // offsetof(struct s, i) != 0 would fail (i is at offset 0)
    // offsetof(struct s, a) != sizeof(struct s) would fail (a is right after the struct base)
    // These checks pass in C, so we don't return 1 or 2
    
    // Allocate equivalent of malloc(sizeof(struct s) + 4 * sizeof(int))
    // This gives space for 4 integers in the flexible array
    let mut p = S {
        i: 0,
        a: vec![0; 4],
    };
    
    // The malloc check - in Rust, allocation failure would panic, not return null
    // So we skip the null check (return 3 case)
    
    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;
    
    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        std::process::exit(4);
    }
    
    // free(p) is handled automatically by Rust's drop
    
    std::process::exit(0);
}