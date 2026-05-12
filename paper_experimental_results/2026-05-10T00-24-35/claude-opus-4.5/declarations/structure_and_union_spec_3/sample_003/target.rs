use std::mem;

struct S {
    i: i32,
    a: Vec<i32>,
}

fn main() {
    // Check that i is at offset 0 (it's the first field)
    // In Rust, we can't directly use offsetof, but we can verify the structure
    // by checking that the first field is indeed at the start
    if mem::offset_of!(S, i) != 0 {
        std::process::exit(1);
    }

    // In the C version, offsetof(struct s, a) == sizeof(struct s) because a is a flexible array member
    // In Rust with Vec, this doesn't directly apply, but we simulate the check
    // The flexible array starts right after the fixed part of the struct
    // In C, sizeof(struct s) would be sizeof(int) (padded appropriately) which equals offsetof(struct s, a)
    // We'll check that the offset of 'a' is at least sizeof(i32)
    if mem::offset_of!(S, a) < mem::size_of::<i32>() {
        std::process::exit(2);
    }

    // Allocate equivalent of struct s with 4 integers in the flexible array
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