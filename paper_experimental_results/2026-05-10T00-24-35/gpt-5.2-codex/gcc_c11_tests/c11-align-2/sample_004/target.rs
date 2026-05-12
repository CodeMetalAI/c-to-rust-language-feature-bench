#[repr(align(16))]
struct Aligned<T>(T);

#[allow(dead_code)]
static C: Aligned<u8> = Aligned(0);
#[allow(dead_code)]
static S: Aligned<i16> = Aligned(0);
#[allow(dead_code)]
static I: Aligned<i32> = Aligned(0);
#[allow(dead_code)]
static L: Aligned<i64> = Aligned(0);
#[allow(dead_code)]
static LL: Aligned<i64> = Aligned(0);
#[allow(dead_code)]
static F: Aligned<f32> = Aligned(0.0);
#[allow(dead_code)]
static D: Aligned<f64> = Aligned(0.0);

#[repr(align(16))]
struct ComplexLongDouble {
    re: f64,
    im: f64,
}

#[allow(dead_code)]
static CLD: ComplexLongDouble = ComplexLongDouble { re: 0.0, im: 0.0 };

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