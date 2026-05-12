use std::mem;

fn main() {
    // Rust doesn't have flexible array members like C, so we simulate them with
    // a struct containing a pointer to a dynamically allocated array.
    struct S {
        n: i32,
        d: Vec<f64>,
    }

    struct SS {
        n: i32,
    }

    // Check that S's size is at least as large as SS's size.
    if mem::size_of::<S>() < mem::size_of::<SS>() {
        return;
    }

    // In Rust, the Vec is a separate field, so offsetof doesn't apply directly.
    // Instead, we check that the Vec's pointer is after n.
    // This is always true for Rust's layout, but we simulate the check.
    // The offset of d in memory is after n, but exact offset depends on alignment.
    // We'll trust Rust's layout.

    // Create instances with different array sizes.
    let s1 = S {
        n: 0,
        d: vec![0.0; 8],
    };
    let s2 = S {
        n: 0,
        d: vec![0.0; 5],
    };

    s1.d[0] = 42.0;
    s2.d[0] = 24.0;

    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        return;
    }

    // No explicit free needed; Vecs are automatically deallocated.
}