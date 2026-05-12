use std::mem;

struct S {
    i: i32,
    a: Vec<i32>,
}

fn main() {
    // Check offsets
    assert_eq!(mem::size_of::<i32>(), 4); // Ensure size of i32 is 4 bytes as in C++
    if mem::size_of::<i32>() != 4 {
        std::process::exit(1);
    }

    // Allocate the structure with additional space for the array
    let mut p = S {
        i: 0,
        a: vec![0; 4], // Allocate space for 4 integers
    };

    // Assign values
    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;

    // Check values
    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        std::process::exit(4);
    }
}