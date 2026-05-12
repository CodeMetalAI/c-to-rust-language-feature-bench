use std::process::abort;

const ALIGN: usize = 16;

#[repr(align(ALIGN))]
struct Align16<T>(T);

#[allow(dead_code)]
static C: Align16<u8> = Align16(0);

#[allow(dead_code)]
static S: Align16<i16> = Align16(0);

#[allow(dead_code)]
static I: Align16<i32> = Align16(0);

#[allow(dead_code)]
static L: Align16<i64> = Align16(0);

#[allow(dead_code)]
static LL: Align16<i64> = Align16(0);

#[allow(dead_code)]
static F: Align16<f32> = Align16(0.0);

#[allow(dead_code)]
static D: Align16<f64> = Align16(0.0);

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct ComplexLongDouble {
    re: f64,
    im: f64,
}

#[allow(dead_code)]
static CLD: Align16<ComplexLongDouble> = Align16(ComplexLongDouble { re: 0.0, im: 0.0 });

#[allow(dead_code)]
static CA: [u8; 10] = [0; 10];

#[allow(dead_code)]
static X: Align16<i32> = Align16(0);

const E: usize = ALIGN;

#[allow(dead_code)]
static Y: Align16<i32> = Align16(0);

#[allow(dead_code)]
fn func() {
    let _auto_ll: Align16<i64> = Align16(0);
}

/* Valid, but useless.  */
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
}