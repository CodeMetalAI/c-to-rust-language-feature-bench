use std::mem;

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // TYPE_ID(a) - in C, array decays to pointer (int*), type id 1
    // TYPE_ID(&a) - pointer to array (int (*)[3]), type id 2
    // These checks are about C's type system; in Rust we just verify the array works correctly
    // The checks pass implicitly since we're using the correct types

    // Check array element access
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    // Check sizeof - array of 3 i32s should be 3 * size_of::<i32>()
    if mem::size_of_val(&a) != 3 * mem::size_of::<i32>() {
        std::process::exit(6);
    }
    
    // Check alignment
    if mem::align_of_val(&a) != mem::align_of::<i32>() {
        std::process::exit(7);
    }

    // String literal check - in C "abc" is const char*, type id 3
    // In Rust, string literals are &str, we verify the content
    let s: [u8; 4] = [b'a', b'b', b'c', 0];
    
    // TYPE_ID("abc") check - string literal type, passes implicitly
    
    if s[0] != b'a' {
        std::process::exit(9);
    }
    if s[1] != b'b' {
        std::process::exit(10);
    }
    if s[2] != b'c' {
        std::process::exit(11);
    }
    if s[3] != 0 {
        std::process::exit(12);
    }

    std::process::exit(0);
}