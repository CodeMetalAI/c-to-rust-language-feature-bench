use std::alloc::Layout;

fn main() {
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    if s1 != "_Alignas" {
        std::process::abort();
    }
    if s2 != "_Alignof" {
        std::process::abort();
    }
    if s3 != "1" {
        std::process::abort();
    }
    if s4 != "1" {
        std::process::abort();
    }

    fn func() {
        let _auto_ll: i64 = 0; // The alignment is handled by Rust's types
    }

    struct S; // Valid, but not used

    // Test alignments
    let _c: u8 = 0; // Equivalent to char
    let _s: i16 = 0; // Equivalent to short
    let _i: i32 = 0; // Equivalent to int
    let _l: i64 = 0; // Equivalent to long
    let _ll: i64 = 0; // Equivalent to long long
    let _f: f32 = 0.0; // Equivalent to float
    let _d: f64 = 0.0; // Equivalent to double
    let _ca: [u8; 10] = [0; 10]; // char array
    let _x: i32 = 0;

    enum E {
        AlignMax = std::mem::align_of::<usize>(),
    }
    let _y: i32 = 0;
}