use std::mem;

struct S {
    i: i32,
    a: Vec<i32>,
}

fn main() {
    // Check that 'i' is at offset 0
    // In Rust, the first field of a struct is at offset 0 (with repr(C) or for simple cases)
    // We simulate this check - in our Rust version, 'i' is the first field
    if mem::offset_of!(S, i) != 0 {
        std::process::exit(1);
    }

    // In C, offsetof(struct s, a) == sizeof(struct s) for flexible array members
    // In Rust, we use a Vec which is a separate allocation, so this check doesn't directly apply
    // However, to preserve behavior, we check that the offset of 'a' is after 'i'
    // The C code checks that the flexible array starts right after the struct
    // For our purposes, we just verify the offset of 'a' is sizeof(i32) (after padding if any)
    if mem::offset_of!(S, a) != mem::size_of::<i32>() {
        std::process::exit(2);
    }

    // Allocate the structure with a vector of 4 integers
    let mut p = Box::new(S {
        i: 0,
        a: vec![0; 4],
    });

    // Set values
    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;

    // Verify values
    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        std::process::exit(4);
    }

    // p is automatically freed when it goes out of scope
    std::process::exit(0);
}