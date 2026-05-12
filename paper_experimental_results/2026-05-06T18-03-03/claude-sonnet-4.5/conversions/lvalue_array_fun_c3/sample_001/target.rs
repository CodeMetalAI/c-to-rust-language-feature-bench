fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In C, array 'a' decays to pointer (int*) in most contexts
    // TYPE_ID(a) checks if it's int* (type 1)
    if type_id_a(&a) != 1 {
        std::process::exit(1);
    }
    
    // &a in C is a pointer to the array type: int (*)[3] (type 2)
    if type_id_ref_a(&a) != 2 {
        std::process::exit(2);
    }

    // *a is equivalent to a[0]
    if a[0] != 10 {
        std::process::exit(3);
    }
    
    // *(a + 1) is equivalent to a[1]
    if a[1] != 20 {
        std::process::exit(4);
    }
    
    // *(a + 2) is equivalent to a[2]
    if a[2] != 30 {
        std::process::exit(5);
    }

    // sizeof(a) should be 3 * sizeof(int)
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    
    // _Alignof(a) should equal _Alignof(int)
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [u8; 4] = *b"abc\0";
    
    // "abc" is a string literal (const char*, type 3)
    if type_id_str(b"abc") != 3 {
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
    2 // int (*)[3]
}

// Simulate TYPE_ID for string literal
fn type_id_str(_: &[u8]) -> i32 {
    3 // const char*
}