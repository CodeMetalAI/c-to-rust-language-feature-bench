use std::cmp::Ordering;

#[repr(align(16))] // Assuming max_align_t is 16, as it often is.  Adjust if needed.
static mut c: char = ' ';

#[repr(align(16))] // Assuming max_align_t is 16, as it often is.  Adjust if needed.
static mut s: i16 = 0;

#[repr(align(4))]
static mut i: i32 = 0;

#[repr(align(16))] // Assuming max_align_t is 16, as it often is.  Adjust if needed.
static mut l: i64 = 0;

#[repr(align(16))] // Assuming max_align_t is 16, as it often is.  Adjust if needed.
static mut ll: i64 = 0;

#[repr(align(4))]
static mut f: f32 = 0.0;

#[repr(align(8))]
static mut d: f64 = 0.0;

#[repr(align(16))] // Assuming max_align_t is 16, as it often is.  Adjust if needed.
static mut cld: std::complex::Complex<f64> = std::complex::Complex { re: 0.0, im: 0.0 };

#[repr(align(1))]
static mut ca: [char; 10] = [' '; 10];

#[repr(align(16))] // Assuming max_align_t is 16, as it often is.  Adjust if needed.
static mut x: i32 = 0;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum e {
    E,
}

#[repr(align(16))] // Assuming max_align_t is 16, as it often is.  Adjust if needed.
static mut y: i32 = 0;

fn func() {
    #[repr(align(16))] // Assuming max_align_t is 16, as it often is.  Adjust if needed.
    let auto_ll: i64 = 0;
}

fn strcmp(s1: &str, s2: &str) -> i32 {
    let mut i = 0;
    while s1.as_bytes()[i] == s2.as_bytes()[i] {
        if s1.as_bytes()[i] == 0 {
            return 0;
        }
        i += 1;
    }
    s1.as_bytes()[i].cmp(&s2.as_bytes()[i]) as i32
}

fn abort() {
    std::process::exit(1);
}

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