use std::mem;
use std::process::abort;

// Define global variables with specific alignments
#[repr(align(align_of::<mem::MaybeUninit<()>>()))]
static mut C: u8 = 0;

#[repr(align(align_of::<mem::MaybeUninit<()>>()))]
static mut S: i16 = 0;

#[repr(align(mem::align_of::<i32>()))]
static mut I: i32 = 0;

#[repr(align(align_of::<mem::MaybeUninit<()>>()))]
static mut L: i64 = 0;

#[repr(align(align_of::<mem::MaybeUninit<()>>()))]
static mut LL: i64 = 0;

#[repr(align(align_of::<mem::MaybeUninit<()>>()))]
static mut F: f32 = 0.0;

#[repr(align(align_of::<mem::MaybeUninit<()>>()))]
static mut D: f64 = 0.0;

#[repr(align(align_of::<mem::MaybeUninit<()>>()))]
static mut CLD: (f64, f64) = (0.0, 0.0); // Placeholder for complex

#[repr(align(mem::align_of::<i32>()))]
static mut CA: [u8; 10] = [0; 10];

#[repr(align(align_of::<mem::MaybeUninit<()>>() + 0))]
static mut X: i32 = 0;

// Function to test alignment
fn func() {
    #[repr(align(align_of::<mem::MaybeUninit<()>>()))]
    let _auto_ll: i64 = 0;
}

fn main() {
    const S1: &str = "_Alignas";
    const S2: &str = "_Alignof";
    const S3: &str = "1";
    const S4: &str = "1";

    if S1 != "_Alignas" {
        unsafe { abort() }
    }
    if S2 != "_Alignof" {
        unsafe { abort() }
    }
    if S3 != "1" {
        unsafe { abort() }
    }
    if S4 != "1" {
        unsafe { abort() }
    }
}