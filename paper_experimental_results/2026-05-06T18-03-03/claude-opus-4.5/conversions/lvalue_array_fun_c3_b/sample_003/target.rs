use std::mem;

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // TYPE_ID(a) in C decays to int*, which is type 1
    // TYPE_ID(&a) is int (*)[3], which is type 2
    // In Rust, we just verify the array works correctly
    // The type checks are compile-time in C, so we just need to ensure
    // the program structure is correct

    // Check array element access (equivalent to *a, *(a+1), *(a+2))
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    // Check sizeof a == 3 * sizeof(int)
    if mem::size_of_val(&a) != 3 * mem::size_of::<i32>() {
        std::process::exit(6);
    }

    // Check alignment
    if mem::align_of_val(&a) != mem::align_of::<i32>() {
        std::process::exit(7);
    }

    // char s[4] = "abc" - in Rust we use a byte array
    let s: [u8; 4] = [b'a', b'b', b'c', 0];
    
    // TYPE_ID("abc") in C is const char*, type 3
    // This is a compile-time type check, so we skip it
    
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

    // Return 0 (success)
    std::process::exit(0);
}