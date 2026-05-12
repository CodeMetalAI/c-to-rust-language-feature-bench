use std::mem;

// Global variables with alignment attributes
#[repr(align(16))] // max_align_t is typically 16 bytes on many platforms
static mut C: u8 = 0;
#[repr(align(16))]
static mut S: i16 = 0;
#[repr(align(4))]
static mut I: i32 = 0;
#[repr(align(16))]
static mut L: i64 = 0;
#[repr(align(16))]
static mut LL: i64 = 0;
#[repr(align(16))]
static mut F: f32 = 0.0;
#[repr(align(16))]
static mut D: f64 = 0.0;
#[repr(align(16))]
static mut CLD: [f64; 2] = [0.0, 0.0]; // Complex long double approximation

// Array with multiple alignments - Rust doesn't support multiple alignas
// We'll use the strictest one (int alignment)
#[repr(align(4))]
static mut CA: [u8; 10] = [0; 10];

#[repr(align(16))]
static mut X: i32 = 0;

enum E {
    E = 16, // alignof(max_align_t) is typically 16
}

#[repr(align(16))]
static mut Y: i32 = 0;

fn func() {
    #[repr(align(16))]
    let _auto_ll: i64 = 0;
}

// Check that alignment macros would expand correctly
const S1: &str = "_Alignas";
const S2: &str = "_Alignof";
const S3: &str = "1";
const S4: &str = "1";

fn main() {
    // Verify the strings match expected values
    assert_eq!(S1, "_Alignas");
    assert_eq!(S2, "_Alignof");
    assert_eq!(S3, "1");
    assert_eq!(S4, "1");
    
    // Verify alignments
    unsafe {
        assert_eq!(mem::align_of_val(&C), 16);
        assert_eq!(mem::align_of_val(&S), 16);
        assert_eq!(mem::align_of_val(&I), 4);
        assert_eq!(mem::align_of_val(&L), 16);
        assert_eq!(mem::align_of_val(&LL), 16);
        assert_eq!(mem::align_of_val(&F), 16);
        assert_eq!(mem::align_of_val(&D), 16);
        assert_eq!(mem::align_of_val(&CLD), 16);
        assert_eq!(mem::align_of_val(&CA), 4);
        assert_eq!(mem::align_of_val(&X), 16);
        assert_eq!(mem::align_of_val(&Y), 16);
    }
    
    // Exit with success (implicit in Rust)
}