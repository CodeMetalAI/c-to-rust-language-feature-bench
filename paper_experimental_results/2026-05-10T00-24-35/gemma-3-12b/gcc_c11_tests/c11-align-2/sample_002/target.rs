use std::cmp::Ordering;
use std::mem::align_of;

#[repr(align(16))] // Assuming max_align_t is 16, as it often is.  This is a reasonable guess.
static mut C: char = ' ';

#[repr(align(16))]
static mut S: short = 0;

#[repr(align(4))]
static mut I: int = 0;

#[repr(align(16))]
static mut L: long = 0;

#[repr(align(16))]
static mut LL: long_long = 0;

#[repr(align(4))]
static mut F: float = 0.0;

#[repr(align(8))]
static mut D: double = 0.0;

#[repr(align(16))]
static mut CLD: _Complex<long_double> = _Complex { re: 0.0, im: 0.0 };

#[repr(align(1))]
static mut CA: [char; 10] = [' '; 10];

#[repr(align(16))]
static mut X: int = 0;

#[derive(Copy, Clone)]
#[repr(C)]
enum E {
    E,
}

#[repr(align(16))]
static mut Y: int = 0;

fn func() {
    #[repr(align(16))]
    let auto_ll: long_long = 0;
}

#[repr(transparent)]
struct S {};

fn main() {
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    if strcmp(s1, s2) != 0 {
        abort();
    }
    if strcmp(s3, s4) != 0 {
        abort();
    }
}

extern "C" {
    fn strcmp(s1: *const i8, s2: *const i8) -> i32;
    fn abort() ;
}