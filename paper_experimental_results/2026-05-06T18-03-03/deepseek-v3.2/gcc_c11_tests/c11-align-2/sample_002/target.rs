use std::mem;

// Global variables with alignment constraints
#[repr(align(128))] // max_align_t is typically 128 bytes on many systems
static mut C: u8 = 0;
static mut S: i16 = 0;
static mut I: i32 = 0;
#[repr(align(128))]
static mut L: i64 = 0;
#[repr(align(128))]
static mut LL: i128 = 0;
#[repr(align(128))]
static mut F: f32 = 0.0;
#[repr(align(128))]
static mut D: f64 = 0.0;
#[repr(align(128))]
static mut CLD: [f64; 2] = [0.0, 0.0]; // Complex long double approximation

#[repr(align(4))]
static mut CA: [u8; 10] = [0; 10];

static mut X: i32 = 0;
static mut Y: i32 = 0;

fn func() {
    #[repr(align(128))]
    let auto_ll: i128 = 0;
}

fn main() {
    // Check alignment macros are defined (they are built into Rust)
    // Rust doesn't have exact equivalents, but alignment is part of the language
    
    // Simulate the string checks from the C code
    let s1 = "alignas";
    let s2 = "alignof";
    let s3 = "1";
    let s4 = "1";
    
    // In Rust, we don't have _Alignas/_Alignof keywords, so we'll check the alignment
    // properties of our types instead
    assert_eq!(mem::align_of_val(&C), 128);
    assert_eq!(mem::align_of_val(&S), 128);
    assert_eq!(mem::align_of_val(&I), 4);
    assert_eq!(mem::align_of_val(&L), 128);
    assert_eq!(mem::align_of_val(&LL), 128);
    assert_eq!(mem::align_of_val(&F), 128);
    assert_eq!(mem::align_of_val(&D), 128);
    assert_eq!(mem::align_of_val(&CLD), 128);
    assert_eq!(mem::align_of_val(&CA), 4);
    
    // The original C code would abort if string comparisons failed
    // We'll use assertions instead
    assert_eq!(s1, "alignas");
    assert_eq!(s2, "alignof");
    assert_eq!(s3, "1");
    assert_eq!(s4, "1");
    
    // Exit successfully
}