use std::process;

const MAX_ALIGN: usize = 16;

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

#[repr(align(16))]
struct ComplexLongDouble {
    _data: [u8; 32],
}
static CLD: Align16<ComplexLongDouble> = Align16(ComplexLongDouble { _data: [0; 32] });

static CA: Align4<[u8; 10]> = Align4([0; 10]);

static X: Align16<i32> = Align16(0);
static Y: Align16<i32> = Align16(0);

fn func() {
    let _auto_ll: Align16<i64> = Align16(0);
}

#[repr(align(1))]
struct Useless;
static _USELESS: Align1<Useless> = Align1(Useless);

fn main() {
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    if s1 != "_Alignas" {
        process::abort();
    }
    if s2 != "_Alignof" {
        process::abort();
    }
    if s3 != "1" {
        process::abort();
    }
    if s4 != "1" {
        process::abort();
    }

    let _ = (
        MAX_ALIGN, &C, &S, &I, &L, &LL, &F, &D, &CLD, &CA, &X, &Y,
    );
    func();
}