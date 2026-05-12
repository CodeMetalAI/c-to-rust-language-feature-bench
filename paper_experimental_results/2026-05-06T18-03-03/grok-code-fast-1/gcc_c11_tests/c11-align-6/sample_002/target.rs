use std::mem;
use std::process;

#[derive(Default)]
#[repr(C)]
struct ComplexFloat {
    re: f32,
    im: f32,
}

#[derive(Default)]
#[repr(C)]
struct ComplexDouble {
    re: f64,
    im: f64,
}

#[derive(Default)]
#[repr(C)]
struct ComplexLongDouble {
    re: f64,
    im: f64,
}

fn main() {
    macro_rules! check_align {
        ($t:ty) => {{
            #[repr(C)]
            struct X {
                c: u8,
                v: $t,
            }
            let x = X::default();
            if mem::align_of::<$t>() > mem::align_of_val(&x.v) {
                process::abort();
            }
        }};
    }

    check_align!(bool);
    check_align!(char);
    check_align!(i8);
    check_align!(u8);
    check_align!(i16);
    check_align!(u16);
    check_align!(i32);
    check_align!(u32);
    check_align!(isize);
    check_align!(usize);
    check_align!(i64);
    check_align!(u64);
    check_align!(f32);
    check_align!(f64);
    check_align!(f64);
    check_align!(ComplexFloat);
    check_align!(ComplexDouble);
    check_align!(ComplexLongDouble);
    process::exit(0);
}