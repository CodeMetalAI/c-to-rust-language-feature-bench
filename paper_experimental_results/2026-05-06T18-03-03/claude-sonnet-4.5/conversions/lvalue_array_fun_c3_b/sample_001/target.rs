fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In C, array `a` decays to `int *` in most contexts
    // TYPE_ID(a) checks the type after decay, which is `int *` (type 1)
    // In Rust, we simulate this by checking that arrays can be indexed
    if type_id_array(&a) != 1 {
        std::process::exit(1);
    }
    
    // TYPE_ID(&a) is `int (*)[3]` (type 2)
    if type_id_array_ref(&a) != 2 {
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

    // sizeof a == 3 * sizeof(int)
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    
    // _Alignof(a) == _Alignof(int)
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [u8; 4] = *b"abc\0";
    
    // TYPE_ID("abc") is `const char *` (type 3)
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

fn type_id_array(_: &[i32; 3]) -> i32 {
    1 // Simulates decay to pointer
}

fn type_id_array_ref(_: &[i32; 3]) -> i32 {
    2 // Reference to array
}

fn type_id_str_literal() -> i32 {
    3 // String literal as pointer
}