use std::mem;

struct S {
    i: i32,
    a: Vec<i32>,
}

fn main() {
    // Check that i is at offset 0 (it's the first field)
    // In Rust, we can't directly use offsetof, but we can verify the structure
    // by checking that the first field is at the beginning
    if mem::offset_of!(S, i) != 0 {
        std::process::exit(1);
    }

    // The second check in C verifies that the flexible array member starts
    // after the struct's fixed portion. In Rust with Vec, this doesn't apply
    // the same way, but we can check that 'a' comes after 'i'
    if mem::offset_of!(S, a) <= mem::offset_of!(S, i) {
        std::process::exit(2);
    }

    // Create the struct with a vector of 4 elements (equivalent to malloc with space for 4 ints)
    let mut p = Box::new(S {
        i: 0,
        a: vec![0; 4],
    });

    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;

    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        std::process::exit(4);
    }

    // p is automatically freed when it goes out of scope
    std::process::exit(0);
}