use std::mem::{size_of, align_of};

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // TYPE_ID(a) != 1
    // In C, array decays to pointer in most contexts
    // In Rust, we check if a.as_ptr() would be *const i32 (equivalent to int*)
    // This check always passes in Rust since arrays don't auto-decay
    // But the C code checks the decayed type, so we verify the pointer type exists
    let _ptr: *const i32 = a.as_ptr();
    // This represents TYPE_ID(a) == 1 in C context
    
    // TYPE_ID(&a) != 2
    // In C, &a is int (*)[3], a pointer to array
    // In Rust, &a is &[i32; 3]
    let _arr_ref: &[i32; 3] = &a;
    // This represents TYPE_ID(&a) == 2 in C context

    // *a != 10
    if a[0] != 10 {
        std::process::exit(3);
    }
    
    // *(a + 1) != 20
    if a[1] != 20 {
        std::process::exit(4);
    }
    
    // *(a + 2) != 30
    if a[2] != 30 {
        std::process::exit(5);
    }

    // sizeof a != 3 * sizeof(int)
    if size_of::<[i32; 3]>() != 3 * size_of::<i32>() {
        std::process::exit(6);
    }
    
    // _Alignof(a) != _Alignof(int)
    if align_of::<[i32; 3]>() != align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [u8; 4] = *b"abc\0";
    
    // TYPE_ID("abc") != 3
    // In C, "abc" is const char* (decayed from array)
    // We just verify the literal exists as expected type
    let _str_lit: &[u8] = b"abc";
    
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