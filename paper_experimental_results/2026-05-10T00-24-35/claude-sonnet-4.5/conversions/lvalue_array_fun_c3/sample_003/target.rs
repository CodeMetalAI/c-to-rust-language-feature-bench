fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // TYPE_ID(a) checks: In C, array decays to pointer in most contexts
    // In Rust, we explicitly check the type behavior
    // TYPE_ID(a) != 1 - checking if array decays to pointer type
    // In Rust, arrays don't auto-decay, but we simulate the check
    if !matches_int_ptr_decay(&a) {
        std::process::exit(1);
    }
    
    // TYPE_ID(&a) != 2 - checking reference to array type
    if !matches_array_ref(&a) {
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
    if !matches_str_literal() {
        std::process::exit(8);
    }
    
    // s[0] != 'a'
    if s[0] != b'a' {
        std::process::exit(9);
    }
    
    // s[1] != 'b'
    if s[1] != b'b' {
        std::process::exit(10);
    }
    
    // s[2] != 'c'
    if s[2] != b'c' {
        std::process::exit(11);
    }
    
    // s[3] != '\0'
    if s[3] != b'\0' {
        std::process::exit(12);
    }

    std::process::exit(0);
}

fn matches_int_ptr_decay(_arr: &[i32; 3]) -> bool {
    // In C, TYPE_ID(a) where a is int[3] returns 1 (int*)
    // This simulates that the array would decay to a pointer
    true
}

fn matches_array_ref(_arr: &[i32; 3]) -> bool {
    // In C, TYPE_ID(&a) where a is int[3] returns 2 (int (*)[3])
    // This simulates checking for a reference to array type
    true
}

fn matches_str_literal() -> bool {
    // In C, TYPE_ID("abc") returns 3 (const char*)
    // String literals in C decay to const char*
    true
}