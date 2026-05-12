use std::cmp::Ordering;
use std::mem::align_of;

#[repr(align(16))] // Assuming max_align_t is 16, as it's commonly the case.  Adjust if needed.
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

    if compare_strings(s1, "_Alignas") != Ordering::Equal {
        std::process::abort();
    }
    if compare_strings(s2, "_Alignof") != Ordering::Equal {
        std::process::abort();
    }
    if compare_strings(s3, "1") != Ordering::Equal {
        std::process::abort();
    }
    if compare_strings(s4, "1") != Ordering::Equal {
        std::process::abort();
    }
}

fn compare_strings(a: &str, b: &str) -> Ordering {
    a.cmp(b)
}

type int = i32;
type long = i64;
type long_long = i64;
type short = i16;
type float = f32;
type double = f64;
type _Complex<T> = std::num::Complex<T>;
type long_double = f64;