use std::process;

#[repr(align(16))]
struct Align16<T>(T);

#[repr(align(4))]
struct Align4<T>(T);

#[repr(align(1))]
struct Align1<T>(T);

static C: Align16<u8> = Align16(0);
static S: Align16<i16> = Align16(0);
static I: Align4<i32> = Align4(0);
static L: Align16<i64> = Align16(0);
static LL: Align16<i64> = Align16(0);
static F: Align16<f32> = Align16(0.0);
static D: Align16<f64> = Align16(0.0);
static CLD: Align16<(f64, f64)> = Align16((0.0, 0.0));
static CA: Align4<[u8; 10]> = Align4([0; 10]);
static X: Align16<i32> = Align16(0);
static Y: Align16<i32> = Align16(0);

fn func() {
    let _auto_ll: Align16<i64> = Align16(0);
}

#[allow(dead_code)]
#[repr(align(1))]
struct SStruct;

static S1: &str = "_Alignas";
static S2: &str = "_Alignof";
static S3: &str = "1";
static S4: &str = "1";

fn main() {
    if S1 != "_Alignas" {
        process::abort();
    }
    if S2 != "_Alignof" {
        process::abort();
    }
    if S3 != "1" {
        process::abort();
    }
    if S4 != "1" {
        process::abort();
    }
}