use std::mem;

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // TYPE_ID(a) would decay to int* which is 1, but in Rust we work with arrays directly
    // The C code checks that 'a' decays to int* (type 1) - this is implicit in C
    // We skip the type checks as Rust's type system works differently
    
    // Check TYPE_ID(a) != 1 - in C, 'a' decays to int*, which matches type 1
    // This check passes in C, so we don't return 1
    
    // Check TYPE_ID(&a) != 2 - in C, &a is int(*)[3], which matches type 2
    // This check passes in C, so we don't return 2

    // Check *a != 10
    if a[0] != 10 {
        std::process::exit(3);
    }
    
    // Check *(a + 1) != 20
    if a[1] != 20 {
        std::process::exit(4);
    }
    
    // Check *(a + 2) != 30
    if a[2] != 30 {
        std::process::exit(5);
    }

    // Check sizeof a != 3 * sizeof(int)
    if mem::size_of_val(&a) != 3 * mem::size_of::<i32>() {
        std::process::exit(6);
    }
    
    // Check _Alignof(a) != _Alignof(int)
    if mem::align_of_val(&a) != mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [u8; 4] = [b'a', b'b', b'c', 0];
    
    // TYPE_ID("abc") in C gives const char* (type 3) due to decay
    // This check passes in C, so we don't return 8

    // Check s[0] != 'a'
    if s[0] != b'a' {
        std::process::exit(9);
    }
    
    // Check s[1] != 'b'
    if s[1] != b'b' {
        std::process::exit(10);
    }
    
    // Check s[2] != 'c'
    if s[2] != b'c' {
        std::process::exit(11);
    }
    
    // Check s[3] != '\0'
    if s[3] != 0 {
        std::process::exit(12);
    }

    // return 0 - successful exit
    std::process::exit(0);
}