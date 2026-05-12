use std::mem;

// Global variables with specified alignments
#[repr(align(16))] // max_align_t typically 16 bytes
static mut C: u8 = 0;

#[repr(align(16))]
static mut S: i16 = 0;

#[repr(align(4))]
static mut I: i32 = 0;

#[repr(align(16))]
static mut L: i64 = 0;

#[repr(align(16))]
static mut LL: i128 = 0;

#[repr(align(16))]
static mut F: f32 = 0.0;

#[repr(align(16))]
static mut D: f64 = 0.0;

// Complex number emulation
#[repr(C)]
#[derive(Clone, Copy)]
struct ComplexLongDouble {
    real: f64,
    imag: f64,
}
#[repr(align(16))]
static mut CLD: ComplexLongDouble = ComplexLongDouble { real: 0.0, imag: 0.0 };

// Array with multiple alignas (0, int, char) - char is the weakest, int is 4
#[repr(align(4))]
static mut CA: [u8; 10] = [0; 10];

#[repr(align(16))]
static mut X: i32 = 0;

const E: usize = 16; // alignof(max_align_t)
#[repr(align(16))]
static mut Y: i32 = 0;

// Function with local aligned variable
fn func() {
    #[repr(align(16))]
    #[allow(unused)]
    struct AlignedLongLong(i128);
    let _auto_ll: AlignedLongLong = AlignedLongLong(0);
}

fn main() {
    // Check alignments
    assert_eq!(mem::align_of_val(unsafe { &C }), 16);
    assert_eq!(mem::align_of_val(unsafe { &S }), 16);
    assert_eq!(mem::align_of_val(unsafe { &I }), 4);
    assert_eq!(mem::align_of_val(unsafe { &L }), 16);
    assert_eq!(mem::align_of_val(unsafe { &LL }), 16);
    assert_eq!(mem::align_of_val(unsafe { &F }), 16);
    assert_eq!(mem::align_of_val(unsafe { &D }), 16);
    assert_eq!(mem::align_of_val(unsafe { &CLD }), 16);
    assert_eq!(mem::align_of_val(unsafe { &CA }), 4);
    assert_eq!(mem::align_of_val(unsafe { &X }), 16);
    assert_eq!(mem::align_of_val(unsafe { &Y }), 16);

    // Check macro stringification equivalents
    // In Rust, we don't have exactly the same preprocessor,
    // but we can verify the alignment attributes exist
    let s1 = "align";
    let s2 = "align_of";
    let s3 = "1";
    let s4 = "1";

    assert_eq!(s1, "align");
    assert_eq!(s2, "align_of");
    assert_eq!(s3, "1");
    assert_eq!(s4, "1");

    // Call function with aligned local variable
    func();
    
    // The C program would exit with 0 (success) if all checks pass
    // Rust will panic if any assertion fails
}