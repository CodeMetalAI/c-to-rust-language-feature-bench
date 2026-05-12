use std::process::abort;

#[repr(align(16))]
struct Align16<T>(T);

static C: Align16<u8> = Align16(0);
static S: Align16<i16> = Align16(0);
static I: Align16<i32> = Align16(0);
static L: Align16<i64> = Align16(0);
static LL: Align16<i64> = Align16(0);
static F: Align16<f32> = Align16(0.0);
static D: Align16<f64> = Align16(0.0);
static CA: [u8; 10] = [0; 10];

const X: i32 = 0;
const Y: i32 = 0;

fn func() {
    let _auto_ll: Align16<i64> = Align16(0);
    let _ = _auto_ll;
}

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
}