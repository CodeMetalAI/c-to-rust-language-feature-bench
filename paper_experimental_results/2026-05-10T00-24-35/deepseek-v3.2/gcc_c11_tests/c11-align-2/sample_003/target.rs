use std::mem;

// Global variables with specific alignments
#[repr(align(128))] // max_align_t is typically 16 or 128, using 128 for safety
static mut C: u8 = 0;
#[repr(align(128))]
static mut S: i16 = 0;
#[repr(align(4))]
static mut I: i32 = 0;
#[repr(align(128))]
static mut L: i64 = 0;
#[repr(align(128))]
static mut LL: i64 = 0;
#[repr(align(128))]
static mut F: f32 = 0.0;
#[repr(align(128))]
static mut D: f64 = 0.0;
#[repr(align(128))]
static mut CLD: (f64, f64) = (0.0, 0.0); // Complex as tuple
#[repr(align(4))]
static mut CA: [u8; 10] = [0; 10];
#[repr(align(128))]
static mut X: i32 = 0;
const E: usize = 128;
#[repr(align(128))]
static mut Y: i32 = 0;

fn func() {
    let _auto_ll: i64 = 0;
    // Rust automatically aligns local variables appropriately
}

fn main() {
    // Check alignments
    assert_eq!(mem::align_of_val(unsafe { &C }), 128);
    assert_eq!(mem::align_of_val(unsafe { &S }), 128);
    assert_eq!(mem::align_of_val(unsafe { &I }), 4);
    assert_eq!(mem::align_of_val(unsafe { &L }), 128);
    assert_eq!(mem::align_of_val(unsafe { &LL }), 128);
    assert_eq!(mem::align_of_val(unsafe { &F }), 128);
    assert_eq!(mem::align_of_val(unsafe { &D }), 128);
    assert_eq!(mem::align_of_val(unsafe { &CLD }), 128);
    assert_eq!(mem::align_of_val(unsafe { &CA }), 4);
    assert_eq!(mem::align_of_val(unsafe { &X }), 128);
    assert_eq!(mem::align_of_val(unsafe { &Y }), 128);

    // Check that alignment macros aren't needed in Rust
    // (alignment is part of the type system)
    
    // Exit successfully (implicit return 0)
}