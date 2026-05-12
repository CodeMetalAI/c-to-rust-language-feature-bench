use std::mem;

#[repr(C, align(16))]
struct MaxAlignT {
    _data: [f64; 2],
}

static mut C: MaxAlignT = MaxAlignT { _data: [0.0; 2] };
static mut S: u16 = 0;
static mut I: i32 = 0;
static mut L: i64 = 0;
static mut LL: i128 = 0;
static mut F: f32 = 0.0;
static mut D: f64 = 0.0;
static mut CLD: [f64; 2] = [0.0; 2];
static mut CA: [u8; 10] = [0; 10];
static mut X: i32 = 0;
static mut Y: i32 = 0;

fn main() {
    let s1 = "alignas";
    let s2 = "alignof";
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
}