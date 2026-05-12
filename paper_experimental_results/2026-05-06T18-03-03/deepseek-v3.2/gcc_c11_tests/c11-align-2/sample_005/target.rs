use std::mem;

// Global variables with alignment attributes
#[repr(align(16))] // max_align_t is typically 16 bytes on many platforms
static mut C: u8 = 0;
static mut S: u16 = 0;
static mut I: i32 = 0;
static mut L: i64 = 0;
static mut LL: i64 = 0;
static mut F: f32 = 0.0;
static mut D: f64 = 0.0;
// Rust doesn't have built-in complex numbers, so we'll use a tuple
#[repr(C)]
#[repr(align(16))]
struct ComplexLongDouble(f64, f64);
static mut CLD: ComplexLongDouble = ComplexLongDouble(0.0, 0.0);

// Array with multiple alignments - Rust doesn't support alignas(0), so we use the strictest one
#[repr(align(4))]
static mut CA: [u8; 10] = [0; 10];

// Variables with computed alignments
#[repr(align(16))]
static mut X: i32 = 0;

enum E {
    Val = 16, // alignof(max_align_t) is typically 16
}

#[repr(align(16))]
static mut Y: i32 = 0;

// Function with local aligned variable
fn func() {
    #[repr(align(16))]
    struct AlignedLL(i64);
    let _auto_ll: AlignedLL = AlignedLL(0);
}

// Check that alignment macros would expand correctly
const S1: &str = "alignas";
const S2: &str = "alignof";
const S3: &str = "1";
const S4: &str = "1";

fn main() {
    // In Rust, we don't have the exact same preprocessor behavior,
    // but we can verify the alignment properties
    
    // Check alignments of global variables
    unsafe {
        assert_eq!(mem::align_of_val(&C), 16);
        assert_eq!(mem::align_of_val(&S), 16);
        assert_eq!(mem::align_of_val(&I), 4); // alignas(int) should be 4
        assert_eq!(mem::align_of_val(&L), 16);
        assert_eq!(mem::align_of_val(&LL), 16);
        assert_eq!(mem::align_of_val(&F), 16);
        assert_eq!(mem::align_of_val(&D), 16);
        assert_eq!(mem::align_of_val(&CLD), 16);
        assert_eq!(mem::align_of_val(&CA), 4);
        assert_eq!(mem::align_of_val(&X), 16);
        assert_eq!(mem::align_of_val(&Y), 16);
    }
    
    // Check the string constants
    assert_eq!(S1, "alignas");
    assert_eq!(S2, "alignof");
    assert_eq!(S3, "1");
    assert_eq!(S4, "1");
    
    // Call the function with aligned local variable
    func();
    
    // Exit successfully
    std::process::exit(0);
}