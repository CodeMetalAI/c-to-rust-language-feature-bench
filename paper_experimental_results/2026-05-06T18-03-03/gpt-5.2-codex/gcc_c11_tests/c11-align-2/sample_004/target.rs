use std::mem::align_of;
use std::process::abort;

const MAX_ALIGN: usize = align_of::<u128>();
const INT_ALIGN: usize = align_of::<i32>();
const CHAR_ALIGN: usize = align_of::<u8>();

#[repr(align(MAX_ALIGN))]
struct AlignMaxChar(u8);
#[repr(align(MAX_ALIGN))]
struct AlignMaxShort(i16);
#[repr(align(INT_ALIGN))]
struct AlignInt(i32);
#[repr(align(MAX_ALIGN))]
struct AlignMaxLong(i64);
#[repr(align(MAX_ALIGN))]
struct AlignMaxLongLong(i64);
#[repr(align(MAX_ALIGN))]
struct AlignMaxFloat(f32);
#[repr(align(MAX_ALIGN))]
struct AlignMaxDouble(f64);
#[repr(align(MAX_ALIGN))]
struct AlignMaxComplexLongDouble([f64; 2]);
#[repr(align(INT_ALIGN))]
struct AlignIntArray([u8; 10]);
#[repr(align(MAX_ALIGN))]
struct AlignMaxInt(i32);

static C: AlignMaxChar = AlignMaxChar(0);
static S: AlignMaxShort = AlignMaxShort(0);
static I: AlignInt = AlignInt(0);
static L: AlignMaxLong = AlignMaxLong(0);
static LL: AlignMaxLongLong = AlignMaxLongLong(0);
static F: AlignMaxFloat = AlignMaxFloat(0.0);
static D: AlignMaxDouble = AlignMaxDouble(0.0);
static CLD: AlignMaxComplexLongDouble = AlignMaxComplexLongDouble([0.0, 0.0]);
static CA: AlignIntArray = AlignIntArray([0u8; 10]);
static X: AlignMaxInt = AlignMaxInt(0);
static Y: AlignMaxInt = AlignMaxInt(0);

fn func() {
    let _auto_ll: AlignMaxLongLong = AlignMaxLongLong(0);
}

#[allow(dead_code)]
struct S;

const S1: &str = "_Alignas";
const S2: &str = "_Alignof";
const S3: &str = "1";
const S4: &str = "1";

fn main() {
    if S1 != "_Alignas" {
        abort();
    }
    if S2 != "_Alignof" {
        abort();
    }
    if S3 != "1" {
        abort();
    }
    if S4 != "1" {
        abort();
    }
    func();
    let _ = (&C, &S, &I, &L, &LL, &F, &D, &CLD, &CA, &X, &Y);
    let _ = CHAR_ALIGN;
}