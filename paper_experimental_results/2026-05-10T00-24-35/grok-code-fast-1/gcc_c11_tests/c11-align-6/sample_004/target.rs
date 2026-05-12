use std::mem;
use std::process;

macro_rules! check_align {
    ($t:ty) => {
        #[repr(C)]
        struct X {
            c: u8,
            v: $t,
        }
        // In C, __alignof__(x.v) is equivalent to align_of::<$t>()
        if mem::align_of::<$t>() > mem::align_of::<$t>() {
            process::abort();
        }
    };
}

fn main() {
    check_align!(bool); // _Bool
    check_align!(i8);   // char / signed char
    check_align!(u8);   // unsigned char
    check_align!(i16);  // signed short
    check_align!(u16);  // unsigned short
    check_align!(i32);  // signed int
    check_align!(u32);  // unsigned int
    check_align!(i64);  // signed long
    check_align!(u64);  // unsigned long
    check_align!(i128); // signed long long
    check_align!(u128); // unsigned long long
    check_align!(f32);  // float
    check_align!(f64);  // double / long double
    check_align!((f32, f32)); // _Complex float
    check_align!((f64, f64)); // _Complex double
    check_align!((f64, f64)); // _Complex long double, assuming f64 for long double
    process::exit(0);
}