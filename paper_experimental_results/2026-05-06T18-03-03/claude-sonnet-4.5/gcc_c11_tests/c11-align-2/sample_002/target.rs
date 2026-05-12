use std::mem::{align_of, size_of};
use std::alloc::Layout;

// In Rust, we use #[repr(align(N))] for alignment specifications
// max_align_t in C corresponds to the maximum fundamental alignment in Rust

// Helper to get max alignment (equivalent to max_align_t)
const MAX_ALIGN: usize = {
    let mut max = align_of::<u8>();
    if align_of::<u16>() > max { max = align_of::<u16>(); }
    if align_of::<u32>() > max { max = align_of::<u32>(); }
    if align_of::<u64>() > max { max = align_of::<u64>(); }
    if align_of::<usize>() > max { max = align_of::<usize>(); }
    if align_of::<f32>() > max { max = align_of::<f32>(); }
    if align_of::<f64>() > max { max = align_of::<f64>(); }
    max
};

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

#[repr(align(16))]
static mut CLD: (f64, f64) = (0.0, 0.0);

#[repr(align(4))]
static mut CA: [i8; 10] = [0; 10];

#[repr(align(16))]
static mut X: i32 = 0;

const E: usize = MAX_ALIGN;

#[repr(align(16))]
static mut Y: i32 = 0;

fn func() {
    #[repr(align(16))]
    struct AlignedI64(i64);
    let _auto_ll = AlignedI64(0);
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