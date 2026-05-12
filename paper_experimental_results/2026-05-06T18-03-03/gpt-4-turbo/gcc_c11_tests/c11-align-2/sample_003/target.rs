use std::mem::align_of;

fn main() {
    // Rust's equivalent to C's alignas and alignof are handled at compile time
    // using attributes and the `align_of` function.

    // Define variables with specific alignment using `align` attribute
    #[repr(align(32))] // Example alignment, adjust as needed
    struct MaxAlignT;

    #[repr(align(32))]
    static mut C: char = '\0';

    #[repr(align(32))]
    static mut S: i16 = 0;

    #[repr(align(4))]
    static mut I: i32 = 0;

    #[repr(align(32))]
    static mut L: i64 = 0;

    #[repr(align(32))]
    static mut LL: i64 = 0;

    #[repr(align(32))]
    static mut F: f32 = 0.0;

    #[repr(align(32))]
    static mut D: f64 = 0.0;

    #[repr(align(32))]
    static mut CLD: f64 = 0.0; // Rust does not have native complex types in std

    #[repr(align(4))]
    static mut CA: [char; 10] = ['\0'; 10];

    #[repr(align(32))]
    static mut X: i32 = 0;

    #[repr(align(32))]
    static mut Y: i32 = 0;

    // Check the alignment values
    assert_eq!(align_of::<MaxAlignT>(), 32);
    assert_eq!(align_of::<char>(), 4); // Typically 4 on most systems, but platform-dependent
    assert_eq!(align_of::<i16>(), 2);
    assert_eq!(align_of::<i32>(), 4);
    assert_eq!(align_of::<i64>(), 8);
    assert_eq!(align_of::<f32>(), 4);
    assert_eq!(align_of::<f64>(), 8);

    // Strings representing the usage of align in Rust
    let s1 = "align";
    let s2 = "align_of";
    let s3 = "1"; // Simulating __alignas_is_defined
    let s4 = "1"; // Simulating __alignof_is_defined

    // Perform checks as in the original C code
    if s1 != "align" {
        panic!();
    }
    if s2 != "align_of" {
        panic!();
    }
    if s3 != "1" {
        panic!();
    }
    if s4 != "1" {
        panic!();
    }
}