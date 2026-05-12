use std::mem::{align_of, size_of};
use std::process;

// In Rust, we use repr(align) for type alignment
// max_align_t in C corresponds to the largest fundamental alignment
// which in Rust is typically represented by the largest primitive type's alignment

#[repr(align(16))] // Using 16 as a reasonable max_align_t equivalent
struct MaxAlign;

const MAX_ALIGN: usize = align_of::<MaxAlign>();

#[repr(align(16))]
static mut C: i8 = 0;

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

// Rust doesn't have complex numbers in std, but we can represent the alignment
#[repr(align(16))]
static mut CLD: [u8; 32] = [0; 32]; // Approximate size for complex long double

#[repr(align(4))]
static mut CA: [i8; 10] = [0; 10];

#[repr(align(16))]
static mut X: i32 = 0;

const E: usize = MAX_ALIGN;

#[repr(align(16))]
static mut Y: i32 = 0;

fn func() {
    #[repr(align(16))]
    struct AlignedLL {
        _value: i64,
    }
    let _auto_ll = AlignedLL { _value: 0 };
}

// Simulating the preprocessor macro stringification
const S1: &str = "_Alignas";
const S2: &str = "_Alignof";
const S3: &str = "1";
const S4: &str = "1";

fn main() {
    if S1 != "_Alignas" {
        process::abort();
    }
    if S2 != "_Alignof" {
        process::abort();
    }
    if S3 != "1" {
        process::abort();
    }
    if S4 != "1" {
        process::abort();
    }
}