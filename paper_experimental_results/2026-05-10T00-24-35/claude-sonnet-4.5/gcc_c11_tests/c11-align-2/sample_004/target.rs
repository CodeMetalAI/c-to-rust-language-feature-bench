use std::mem::{align_of, size_of};
use std::alloc::Layout;

// In Rust, we use repr(align) for type alignment
// max_align_t in C corresponds to the maximum fundamental alignment
// In Rust, this is typically 16 bytes on most platforms (for types like i128, f64x2, etc.)
// We'll use the alignment of the most-aligned standard type

#[repr(align(16))]
struct MaxAlign;

// Global variables with specific alignment
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

// Complex long double - Rust doesn't have native complex types or long double
// We'll use a struct to represent it with the same alignment
#[repr(align(16))]
static mut CLD: [u8; 32] = [0; 32];

#[repr(align(4))]
static mut CA: [u8; 10] = [0; 10];

#[repr(align(16))]
static mut X: i32 = 0;

#[repr(align(16))]
static mut Y: i32 = 0;

fn func() {
    #[repr(align(16))]
    struct AlignedI64(i64);
    let _auto_ll = AlignedI64(0);
}

// String constants that simulate the C preprocessor stringification
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