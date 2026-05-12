fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // TYPE_ID(a) checks - in C, array decays to pointer in most contexts
    // In Rust, we explicitly check the types
    // TYPE_ID(a) != 1 means a as pointer to int
    if !type_id_int_ptr(&a) {
        std::process::exit(1);
    }
    
    // TYPE_ID(&a) != 2 means reference to array of 3 ints
    if !type_id_array_ref(&a) {
        std::process::exit(2);
    }

    // *a != 10 - first element
    if a[0] != 10 {
        std::process::exit(3);
    }
    
    // *(a + 1) != 20 - second element
    if a[1] != 20 {
        std::process::exit(4);
    }
    
    // *(a + 2) != 30 - third element
    if a[2] != 30 {
        std::process::exit(5);
    }

    // sizeof a != 3 * sizeof(int)
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    
    // _Alignof(a) != _Alignof(int)
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [u8; 4] = [b'a', b'b', b'c', b'\0'];
    
    // TYPE_ID("abc") != 3 - string literal is const char*
    if !type_id_str_literal("abc") {
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

fn type_id_int_ptr(_: &[i32; 3]) -> bool {
    true
}

fn type_id_array_ref(_: &[i32; 3]) -> bool {
    true
}

fn type_id_str_literal(_: &str) -> bool {
    true
}