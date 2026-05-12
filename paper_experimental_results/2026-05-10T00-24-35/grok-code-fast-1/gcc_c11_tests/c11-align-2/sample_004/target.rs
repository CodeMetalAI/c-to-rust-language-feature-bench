use std::mem;

static C: u8 = 0;

#[repr(align(2))]
static S: i16 = 0;

#[repr(align(4))]
static I: i32 = 0;

#[repr(align(8))]
static L: i64 = 0;

#[repr(align(8))]
static LL: i64 = 0;

#[repr(align(4))]
static F: f32 = 0.0;

#[repr(align(8))]
static D: f64 = 0.0;

#[repr(align(8))]
static CLD: (f64, f64) = (0.0, 0.0);

static CA: [u8; 10] = [0; 10];

static X: i32 = 0;

#[repr(u32)]
enum E {
    E = mem::align_of::<()>(),
}

static Y: i32 = 0;

struct S;

fn func() {
    let auto_ll: i64 = 0;
}

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
    func();
}