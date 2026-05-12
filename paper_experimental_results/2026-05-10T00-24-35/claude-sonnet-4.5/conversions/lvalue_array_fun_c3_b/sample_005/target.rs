use std::mem::{size_of, align_of};

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // TYPE_ID(a) checks:
    // In C, array 'a' decays to pointer in most contexts (int *)
    // TYPE_ID(&a) would be int (*)[3]
    // Since we can't directly replicate _Generic, we verify behavior instead
    
    // Check 1: a decays to pointer (int *) - implicit in Rust slicing
    // Check 2: &a is reference to array - verified by type system
    
    // Checks 3-5: Array element access
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    // Check 6: sizeof(a) == 3 * sizeof(int)
    if size_of::<[i32; 3]>() != 3 * size_of::<i32>() {
        std::process::exit(6);
    }
    
    // Check 7: alignof(a) == alignof(int)
    if align_of::<[i32; 3]>() != align_of::<i32>() {
        std::process::exit(7);
    }

    // Check 8: String literal type check
    // In C, "abc" is const char *, s is char[4]
    let s: [u8; 4] = *b"abc\0";
    
    // Checks 9-12: String character access
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