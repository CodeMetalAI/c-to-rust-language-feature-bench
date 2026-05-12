use std::mem::align_of;

macro_rules! CHECK_ALIGN {
    ($T:ty) => {
        {
            struct S {
                c: u8,
                v: $T,
            }
            if align_of::<$T>() > std::mem::align_of_val(&S { c: 0, v: unsafe { std::mem::zeroed() } }.v) {
                std::process::exit(1);
            }
        }
    };
}

fn main() {
    CHECK_ALIGN!(bool);
    CHECK_ALIGN!(i8);
    CHECK_ALIGN!(u8);
    CHECK_ALIGN!(i16);
    CHECK_ALIGN!(u16);
    CHECK_ALIGN!(i32);
    CHECK_ALIGN!(u32);
    CHECK_ALIGN!(i64);
    CHECK_ALIGN!(u64);
    CHECK_ALIGN!(i128);
    CHECK_ALIGN!(u128);
    CHECK_ALIGN!(f32);
    CHECK_ALIGN!(f64);
    CHECK_ALIGN!(f64); // long double typically maps to f64 in Rust
    CHECK_ALIGN!(num::Complex<f32>);
    CHECK_ALIGN!(num::Complex<f64>);
    CHECK_ALIGN!(num::Complex<f64>); // Complex long double also maps to Complex<f64>
}