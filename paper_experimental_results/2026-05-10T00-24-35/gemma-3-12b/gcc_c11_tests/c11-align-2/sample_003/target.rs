use std::cmp::Ordering;
use std::mem::align_of;

#[repr(align(16))] // Assuming max_align_t is 16 for x86_64
#[repr(align(16))]
static mut c: char = ' ';

#[repr(align(16))]
#[repr(align(16))]
static mut s: i16 = 0;

#[repr(align(4))]
#[repr(align(4))]
static mut i: i32 = 0;

#[repr(align(16))]
#[repr(align(16))]
static mut l: i64 = 0;

#[repr(align(16))]
#[repr(align(16))]
static mut ll: i64 = 0;

#[repr(align(4))]
#[repr(align(4))]
static mut f: f32 = 0.0;

#[repr(align(8))]
#[repr(align(8))]
static mut d: f64 = 0.0;

#[repr(align(16))]
#[repr(align(16))]
static mut cld: std::complex::Complex<f64> = std::complex::Complex { re: 0.0, im: 0.0 };

#[repr(align(1))]
static mut ca: [char; 10] = [' '; 10];

#[repr(align(16))]
static mut x: i32 = 0;

#[derive(Copy, Clone)]
#[repr(C)]
enum e { E = 16 } // Assuming max_align_t is 16

#[repr(align(16))]
static mut y: i32 = 0;

fn func() {
    #[repr(align(16))]
    let auto_ll: i64 = 0;
}

fn main() {
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