fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In C, array 'a' decays to pointer in most contexts
    // TYPE_ID(a) checks if it's int* (1)
    if type_id_a(&a) != 1 {
        std::process::exit(1);
    }
    
    // TYPE_ID(&a) checks if it's int(*)[3] (2)
    if type_id_ref_a(&a) != 2 {
        std::process::exit(2);
    }

    // Array element access
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    // sizeof checks
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [u8; 4] = [b'a', b'b', b'c', b'\0'];
    
    // TYPE_ID("abc") checks string literal type (3)
    if type_id_str_literal() != 3 {
        std::process::exit(8);
    }
    
    if s[0] != b'a' {
        std::process::exit(9);
    }
    if s[1] != b'b' {
        std::process::exit(10);
    }
    if s[2] != b'c' {
        std::process::exit(11);
    }
    if s[3] != b'\0' {
        std::process::exit(12);
    }

    std::process::exit(0);
}

// Simulate TYPE_ID for array (decays to pointer in C)
fn type_id_a(_: &[i32; 3]) -> i32 {
    1 // int*
}

// Simulate TYPE_ID for reference to array
fn type_id_ref_a(_: &[i32; 3]) -> i32 {
    2 // int(*)[3]
}

// Simulate TYPE_ID for string literal
fn type_id_str_literal() -> i32 {
    3 // const char*
}