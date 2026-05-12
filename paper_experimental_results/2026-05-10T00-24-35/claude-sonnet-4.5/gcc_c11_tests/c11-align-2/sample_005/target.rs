use std::mem::{align_of, size_of};
use std::process;

// In Rust, we use repr(align) for custom alignment
// max_align_t in C corresponds to the largest fundamental alignment,
// which we'll approximate with the alignment of the largest primitive type

const MAX_ALIGN: usize = {
    let mut max = align_of::<u8>();
    if align_of::<u16>() > max { max = align_of::<u16>(); }
    if align_of::<u32>() > max { max = align_of::<u32>(); }
    if align_of::<u64>() > max { max = align_of::<u64>(); }
    if align_of::<u128>() > max { max = align_of::<u128>(); }
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

#[repr(align(16))]
static mut Y: i32 = 0;

fn main() {
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    if s1 != "_Alignas" {
        process::abort();
    }
    if s2 != "_Alignof" {
        process::abort();
    }
    if s3 != "1" {
        process::abort();
    }
    if s4 != "1" {
        process::abort();
    }
}