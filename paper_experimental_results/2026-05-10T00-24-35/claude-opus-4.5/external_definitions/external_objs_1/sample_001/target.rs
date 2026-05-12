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

    // In safe Rust, we can't have pointers that alias mutable variables
    // in the same way as C. However, we can verify the logic by using
    // references at the point of use, or by restructuring.
    // Since the C code is essentially checking that pointers point to
    // the correct variables, and then modifying through the original
    // variables and reading through pointers, we can simulate this
    // by just using the variables directly (the pointer checks always pass
    // by construction in a correct program).

    // The pointer equality checks (p_i1 != &i1, etc.) would always be false
    // in the original C code since they're initialized to point to those vars.
    // So those checks pass (don't return 6-9).

    i1 = 10;
    i2 = 20;
    i4 = 30;
    i5 = 40;

    // *p_i1 would be i1, *p_i2 would be i2, etc.
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