use std::mem::{align_of, align_of_val};
use std::process::{abort, exit};

macro_rules! check_align {
    ($t:ty) => {{
        #[repr(C)]
        struct X {
            c: u8,
            v: $t,
        }
        let x = X {
            c: 0,
            v: Default::default(),
        };
        if align_of::<$t>() > align_of_val(&x.v) {
            abort();
        }
    }};
}

fn main() {
    check_align!(bool);
    check_align!(u8); // for char
    check_align!(i8); // signed char
    check_align!(u8); // unsigned char
    check_align!(i16); // signed short
    check_align!(u16); // unsigned short
    check_align!(i32); // signed int
    check_align!(u32); // unsigned int
    check_align!(i64); // signed long
    check_align!(u64); // unsigned long
    check_align!(i64); // signed long long
    check_align!(u64); // unsigned long long
    check_align!(f32); // float
    check_align!(f64); // double
    check_align!(f64); // long double
    check_align!((f32, f32)); // _Complex float
    check_align!((f64, f64)); // _Complex double
    check_align!((f64, f64)); // _Complex long double
    exit(0);
}