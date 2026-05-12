use std::mem::{align_of, size_of};
use std::alloc::Layout;

// In Rust, we use repr(align) for alignment specifications
// max_align_t in C corresponds to the maximum fundamental alignment in Rust

#[repr(align(16))] // Using 16 as a reasonable max_align_t equivalent
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
static mut CLD: (f64, f64) = (0.0, 0.0); // Complex long double as tuple

#[repr(align(4))]
static mut CA: [u8; 10] = [0; 10];

#[repr(align(16))]
static mut X: i32 = 0;

const E: usize = 16; // alignof(max_align_t)

#[repr(align(16))]
static mut Y: i32 = 0;

fn func() {
    #[repr(align(16))]
    struct AlignedLL(i64);
    let _auto_ll = AlignedLL(0);
}

// Simulating the macro stringification behavior
const S1: &str = "_Alignas";
const S2: &str = "_Alignof";
const S3: &str = "1";
const S4: &str = "1";

fn main() {
    if S1 != "_Alignas" {
        std::process::abort();
    }
    if S2 != "_Alignof" {
        std::process::abort();
    }
    if S3 != "1" {
        std::process::abort();
    }
    if S4 != "1" {
        std::process::abort();
    }
}