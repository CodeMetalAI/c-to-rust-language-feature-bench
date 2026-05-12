fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In Rust, arrays coerce to slices, but we need to check the actual type.
    // For TYPE_ID(a) != 1: In C, `a` decays to `int*` in this context.
    // In Rust, we'll check if `a` is an array of 3 i32.
    if std::any::TypeId::of::<[i32; 3]>() != std::any::TypeId::of::<&i32>() {
        // This condition is always true in Rust, but we need to match the C behavior.
        // The C code expects TYPE_ID(a) == 1 (int*). In Rust, we can't directly replicate
        // _Generic, so we'll simulate the checks.
        // We'll use a helper to mimic the TYPE_ID behavior.
        if type_id(&a) != 1 {
            std::process::exit(1);
        }
    }

    // For TYPE_ID(&a) != 2: In C, &a is int (*)[3]
    if type_id(&&a) != 2 {
        std::process::exit(2);
    }

    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [char; 4] = ['a', 'b', 'c', '\0'];
    // For TYPE_ID("abc") != 3: In C, string literal is const char*
    // In Rust, string literals are &str, but we can check the type.
    if type_id("abc") != 3 {
        std::process::exit(8);
    }

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

    std::process::exit(0);
}

// Helper function to simulate the TYPE_ID macro
fn type_id<T>(_x: T) -> u32 {
    // Use TypeId to distinguish types, but we need to map to the C constants.
    let type_id = std::any::TypeId::of::<T>();
    
    if type_id == std::any::TypeId::of::<&i32>() {
        1
    } else if type_id == std::any::TypeId::of::<&[i32; 3]>() {
        2
    } else if type_id == std::any::TypeId::of::<&str>() {
        3
    } else if type_id == std::any::TypeId::of::<[char; 4]>() {
        4
    } else {
        99
    }
}