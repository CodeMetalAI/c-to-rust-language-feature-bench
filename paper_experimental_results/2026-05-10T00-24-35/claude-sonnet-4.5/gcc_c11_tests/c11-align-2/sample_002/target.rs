use std::mem::{align_of, size_of};

#[repr(align(16))]
struct MaxAlignT {
    _data: [u8; 16],
}

#[repr(C, align(16))]
static mut C: i8 = 0;

#[repr(C, align(16))]
static mut S: i16 = 0;

#[repr(C, align(4))]
static mut I: i32 = 0;

#[repr(C, align(16))]
static mut L: i64 = 0;

#[repr(C, align(16))]
static mut LL: i64 = 0;

#[repr(C, align(16))]
static mut F: f32 = 0.0;

#[repr(C, align(16))]
static mut D: f64 = 0.0;

#[repr(C, align(16))]
static mut CLD: (f64, f64) = (0.0, 0.0);

#[repr(C, align(4))]
static mut CA: [i8; 10] = [0; 10];

#[repr(C, align(16))]
static mut X: i32 = 0;

#[repr(C, align(16))]
static mut Y: i32 = 0;

fn func() {
    #[repr(align(16))]
    struct AutoLl {
        _value: i64,
    }
    let _auto_ll = AutoLl { _value: 0 };
}

fn main() {
    const S1: &str = "_Alignas";
    const S2: &str = "_Alignof";
    const S3: &str = "1";
    const S4: &str = "1";

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