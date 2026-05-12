fn main() {
    let mut i1: i32 = 1;
    let mut i2: i32 = 2;
    let i3: i32 = 3;
    let mut i4: i32 = 0;
    let mut i5: i32 = 0;

    if i1 != 1 {
        std::process::exit(1);
    }
    if i2 != 2 {
        std::process::exit(2);
    }
    if i3 != 3 {
        std::process::exit(3);
    }

    if i4 != 0 {
        std::process::exit(4);
    }
    if i5 != 0 {
        std::process::exit(5);
    }

    // In safe Rust, we can't have raw pointers like in C.
    // The pointer comparisons in the original code are checking that
    // the pointers point to the same variables they were initialized with.
    // Since we're using references in Rust, this is always true by construction.
    // We'll use references to verify the aliasing behavior.

    let p_i1 = &mut i1 as *mut i32;
    let p_i2 = &mut i2 as *mut i32;
    let p_i4 = &mut i4 as *mut i32;
    let p_i5 = &mut i5 as *mut i32;

    // The pointer equality checks are trivially true since we just assigned them
    // We verify this by checking they point to the same address
    if p_i1 != &mut i1 as *mut i32 {
        std::process::exit(6);
    }
    if p_i2 != &mut i2 as *mut i32 {
        std::process::exit(7);
    }
    if p_i4 != &mut i4 as *mut i32 {
        std::process::exit(8);
    }
    if p_i5 != &mut i5 as *mut i32 {
        std::process::exit(9);
    }

    i1 = 10;
    i2 = 20;
    i4 = 30;
    i5 = 40;

    // Verify the values through the variables directly
    // (equivalent to dereferencing pointers that alias the same memory)
    if i1 != 10 {
        std::process::exit(10);
    }
    if i2 != 20 {
        std::process::exit(11);
    }
    if i4 != 30 {
        std::process::exit(12);
    }
    if i5 != 40 {
        std::process::exit(13);
    }

    std::process::exit(0);
}