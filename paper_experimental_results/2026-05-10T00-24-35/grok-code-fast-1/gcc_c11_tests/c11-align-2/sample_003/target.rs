use std::process::exit;

#[repr(align(16))]
struct AlignedU8(u8);

#[repr(align(16))]
struct AlignedI16(i16);

#[repr(align(4))]
struct AlignedI32(i32);

#[repr(align(16))]
struct AlignedI64(i64);

#[repr(align(16))]
struct AlignedUsize(usize);

#[repr(align(16))]
struct AlignedF32(f32);

#[repr(align(16))]
struct AlignedF64(f64);

#[repr(align(16))]
struct AlignedC128(num_complex::Complex<f64>); // Approximation for _Complex long double

static c: AlignedU8 = AlignedU8(0);
static s: AlignedI16 = AlignedI16(0);
static i: AlignedI32 = AlignedI32(0);
static l: AlignedUsize = AlignedUsize(0);
static ll: AlignedI64 = AlignedI64(0);
static f: AlignedF32 = AlignedF32(0.0);
static d: AlignedF64 = AlignedF64(0.0);
static cld: AlignedC128 = AlignedC128(num_complex::Complex::new(0.0, 0.0));
static ca: [u8; 10] = [0; 10];
static x: AlignedI32 = AlignedI32(0);
static y: AlignedI32 = AlignedI32(0);

fn func() {
    let _auto_ll: AlignedI64 = AlignedI64(0);
}

#[repr(align(1))]
struct S;

const s1: &str = "_Alignas";
const s2: &str = "_Alignof";
const s3: &str = "1";
const s4: &str = "1";

fn main() {
    if s1 != "_Alignas" {
        exit(1);
    }
    if s2 != "_Alignof" {
        exit(1);
    }
    if s3 != "1" {
        exit(1);
    }
    if s4 != "1" {
        exit(1);
    }
}