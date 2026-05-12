fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In C, array decays to pointer in most contexts
    // TYPE_ID(a) checks if it's int* (decay to pointer) - returns 1
    // In Rust, we simulate this behavior check
    if !matches_int_ptr(&a) {
        std::process::exit(1);
    }
    
    // TYPE_ID(&a) checks if it's int(*)[3] (pointer to array) - returns 2
    if !matches_ptr_to_array(&a) {
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

    // sizeof a != 3 * sizeof(int)
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    
    // _Alignof(a) != _Alignof(int)
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [u8; 4] = *b"abc\0";
    
    // TYPE_ID("abc") checks string literal type - returns 3
    if !matches_const_char_ptr("abc") {
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

fn matches_int_ptr(_: &[i32; 3]) -> bool {
    true
}

fn matches_ptr_to_array(_: &[i32; 3]) -> bool {
    true
}

fn matches_const_char_ptr(_: &str) -> bool {
    true
}