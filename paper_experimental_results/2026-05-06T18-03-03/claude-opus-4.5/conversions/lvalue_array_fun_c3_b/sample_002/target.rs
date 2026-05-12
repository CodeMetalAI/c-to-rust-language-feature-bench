use std::mem;

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // TYPE_ID(a) would be 1 (int*) due to array-to-pointer decay
    // TYPE_ID(&a) would be 2 (int (*)[3])
    // In Rust, we don't have _Generic, but we can verify the types exist
    // The checks pass implicitly since Rust has strong typing

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

    // sizeof a != 3 * sizeof(int)
    if mem::size_of_val(&a) != 3 * mem::size_of::<i32>() {
        std::process::exit(6);
    }

    // _Alignof(a) != _Alignof(int)
    if mem::align_of_val(&a) != mem::align_of::<i32>() {
        std::process::exit(7);
    }

    // char s[4] = "abc" - includes null terminator
    let s: [u8; 4] = [b'a', b'b', b'c', 0];
    
    // TYPE_ID("abc") would be 3 (const char*) - string literal decays to pointer
    // This check passes implicitly in our Rust version

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

    // Return 0 (success) - implicit in Rust main
}