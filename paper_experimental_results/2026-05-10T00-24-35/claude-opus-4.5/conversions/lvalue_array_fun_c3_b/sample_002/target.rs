use std::mem::{size_of, align_of};

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // TYPE_ID checks are compile-time type checks in C
    // In Rust, the types are what they are, so we just verify the array properties

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

    // Check sizeof equivalent
    if size_of::<[i32; 3]>() != 3 * size_of::<i32>() {
        std::process::exit(6);
    }
    
    // Check alignof equivalent
    if align_of::<[i32; 3]>() != align_of::<i32>() {
        std::process::exit(7);
    }

    // Character array initialization from string
    let s: [u8; 4] = [b'a', b'b', b'c', 0];
    
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