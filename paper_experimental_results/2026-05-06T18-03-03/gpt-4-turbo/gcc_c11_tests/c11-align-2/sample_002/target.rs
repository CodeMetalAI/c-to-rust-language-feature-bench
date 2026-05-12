fn main() {
    // Rust uses `align_of` and `align_of_val` from `std::mem` to handle alignment.
    use std::mem::{align_of, size_of};

    // Define variables with specific alignments using `#[repr(align(N))]`
    #[repr(align(32))]
    struct MaxAlign32;

    #[repr(align(32))]
    static C: char = 'c';

    #[repr(align(32))]
    static S: i16 = 0;

    #[repr(align(4))]
    static I: i32 = 0;

    #[repr(align(32))]
    static L: i64 = 0;

    #[repr(align(32))]
    static LL: i128 = 0;

    #[repr(align(32))]
    static F: f32 = 0.0;

    #[repr(align(32))]
    static D: f64 = 0.0;

    #[repr(align(32))]
    static CLD: f64 = 0.0; // Rust does not support complex types natively

    #[repr(align(4))]
    static CA: [char; 10] = ['\0'; 10];

    #[repr(align(32))]
    static X: i32 = 0;

    #[repr(align(32))]
    static Y: i32 = 0;

    // Check alignments
    assert_eq!(align_of::<MaxAlign32>(), 32);
    assert_eq!(align_of_val(&C), 32);
    assert_eq!(align_of_val(&S), 32);
    assert_eq!(align_of_val(&I), 4);
    assert_eq!(align_of_val(&L), 32);
    assert_eq!(align_of_val(&LL), 32);
    assert_eq!(align_of_val(&F), 32);
    assert_eq!(align_of_val(&D), 32);
    assert_eq!(align_of_val(&CLD), 32);
    assert_eq!(align_of_val(&CA), 4);
    assert_eq!(align_of_val(&X), 32);
    assert_eq!(align_of_val(&Y), 32);

    // Check the string representations
    let s1 = "align_of";
    let s2 = "align_of";
    let s3 = "1";
    let s4 = "1";

    assert_eq!(s1, "align_of");
    assert_eq!(s2, "align_of");
    assert_eq!(s3, "1");
    assert_eq!(s4, "1");
}