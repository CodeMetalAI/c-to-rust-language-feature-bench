use std::ffi::CString;
use std::mem::align_of;

// Global variables with specified alignment
#[repr(align(align_of::<std::ffi::c_void>()))]
static C: [u8; 1] = [0];

#[repr(align(align_of::<std::ffi::c_void>()))]
static S: i16 = 0;

#[repr(align(align_of::<i32>()))]
static I: i32 = 0;

#[repr(align(align_of::<std::ffi::c_void>()))]
static L: i64 = 0;

#[repr(align(align_of::<std::ffi::c_void>()))]
static LL: i64 = 0;

#[repr(align(align_of::<std::ffi::c_void>()))]
static F: f32 = 0.0;

#[repr(align(align_of::<std::ffi::c_void>()))]
static D: f64 = 0.0;

#[repr(align(align_of::<std::ffi::c_void>()))]
static CLD: [f64; 2] = [0.0, 0.0];

#[repr(align(align_of::<i32>()))]
static CA: [u8; 10] = [0; 10];

#[repr(align(align_of::<std::ffi::c_void>()))]
static X: i32 = 0;

enum E {
    E = align_of::<std::ffi::c_void>() as isize,
}

#[repr(align(E::E as usize))]
static Y: i32 = -0;

fn func() {
    #[repr(align(align_of::<std::ffi::c_void>()))]
    let auto_ll: i64 = 0;
}

fn main() {
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    if s1 != "_Alignas" {
        std::process::exit(1);
    }
    if s2 != "_Alignof" {
        std::process::exit(1);
    }
    if s3 != "1" {
        std::process::exit(1);
    }
    if s4 != "1" {
        std::process::exit(1);
    }

    std::process::exit(0);
}