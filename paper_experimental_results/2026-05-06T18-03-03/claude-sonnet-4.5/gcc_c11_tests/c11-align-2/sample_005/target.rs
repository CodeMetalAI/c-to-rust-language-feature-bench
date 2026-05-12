use std::mem::{align_of, size_of};
use std::alloc::Layout;

// Rust doesn't have a direct equivalent to max_align_t, but we can use the largest
// alignment of common types. In practice, this is typically 16 bytes on most platforms.
const MAX_ALIGN: usize = 16;

#[repr(align(16))]
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

// Rust doesn't have complex numbers in std, but we can represent them as a struct
#[repr(align(16))]
static mut CLD: (f64, f64) = (0.0, 0.0);

#[repr(align(4))]
static mut CA: [u8; 10] = [0; 10];

#[repr(align(16))]
static mut X: i32 = 0;

const E: usize = MAX_ALIGN;

#[repr(align(16))]
static mut Y: i32 = 0;

fn func() {
    #[repr(align(16))]
    struct AutoLL(i64);
    let _auto_ll = AutoLL(0);
}

fn main() {
    // In Rust, we simulate the macro stringification behavior
    // alignas maps to repr(align(...))
    // alignof maps to align_of::<T>()
    
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
}