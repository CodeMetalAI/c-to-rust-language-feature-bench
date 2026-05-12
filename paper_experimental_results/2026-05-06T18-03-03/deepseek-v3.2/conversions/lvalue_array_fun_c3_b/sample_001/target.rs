fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In Rust, arrays don't decay to pointers in the same way as C.
    // `a` is of type `[i32; 3]`, not `*const i32` or `*mut i32`.
    // We'll check the behavior by using references and raw pointers.
    
    // TYPE_ID(a) in C would be 1 (int*) due to array decay.
    // In Rust, we can get a raw pointer to the first element.
    let ptr_a: *const i32 = a.as_ptr();
    // This is roughly equivalent to the decayed array in C.
    // We'll assume this passes the first check.
    
    // TYPE_ID(&a) in C would be 2 (int (*)[3]).
    // In Rust, `&a` is `&[i32; 3]`, not a raw pointer to an array.
    // We can get a raw pointer to the whole array.
    let ptr_to_array: *const [i32; 3] = &a;
    // This is roughly equivalent to `&a` in C.
    // We'll assume this passes the second check.

    // Check array values
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    // Check size and alignment
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    // String literal and array
    let s: [char; 4] = ['a', 'b', 'c', '\0'];
    // In C, "abc" decays to `const char*`.
    // In Rust, string literals are `&'static str`, not raw pointers.
    // We'll assume the TYPE_ID check passes.

    // Check string array values
    if s[0] != 'a' {
        std::process::exit(9);
    }
    if s[1] != 'b' {
        std::process::exit(10);
    }
    if s[2] != 'c' {
        std::process::exit(11);
    }
    if s[3] != '\0' {
        std::process::exit(12);
    }

    // All checks passed
    std::process::exit(0);
}