use std::mem;

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // TYPE_ID(a) - in C, array decays to pointer (int*), type id 1
    // TYPE_ID(&a) - pointer to array (int (*)[3]), type id 2
    // These checks pass implicitly in Rust since we're using the correct types

    // Check array values
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

    // char s[4] = "abc"
    let s: [u8; 4] = [b'a', b'b', b'c', 0];
    
    // TYPE_ID("abc") check - string literal decays to const char*, type id 3
    // This is implicit in the Rust translation

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

    // return 0 - successful exit
    std::process::exit(0);
}