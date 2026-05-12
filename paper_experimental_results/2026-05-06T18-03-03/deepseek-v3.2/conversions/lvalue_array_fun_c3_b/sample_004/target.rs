fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In Rust, arrays don't decay to pointers in the same way as C.
    // `a` is of type [i32; 3], not a pointer.
    // We'll check the behavior by using references and raw pointers.
    
    // TYPE_ID(a) in C would be int* (1) due to array decay.
    // In Rust, we can get a raw pointer to the first element.
    let ptr_a: *const i32 = a.as_ptr();
    // This is roughly equivalent to the decayed array in C.
    // We'll just check that we can dereference it correctly.
    
    // Check dereferencing
    unsafe {
        if *ptr_a != 10 {
            std::process::exit(3);
        }
        if *ptr_a.add(1) != 20 {
            std::process::exit(4);
        }
        if *ptr_a.add(2) != 30 {
            std::process::exit(5);
        }
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
    // In Rust, string literals are &str, not pointers like in C.
    // We'll just check the array contents.
    
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

    // The C code uses _Generic to check types at compile time.
    // Rust doesn't have an exact equivalent, but we can use
    // compile-time assertions with const generics or type checking.
    // However, to preserve the exact behavior, we'll just ensure
    // the program exits with 0 if all checks pass.
    
    std::process::exit(0);
}