use std::mem;

macro_rules! check_align {
    ($t:ty) => {
        #[repr(C)]
        struct S {
            c: u8,
            v: $t,
        }
        let align_t = mem::align_of::<$t>();
        // In the C code, __alignof__(x.v) is the alignment of the type, so this check is always false
        if align_t > align_t {
            std::process::abort();
        }
    };
}

fn main() {
    check_align!(bool);
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
    check_align!(f64); // approximating long double as f64
    check_align!((f32, f32)); // approximating _Complex float
    check_align!((f64, f64)); // approximating _Complex double
    check_align!((f64, f64)); // approximating _Complex long double
    std::process::exit(0);
}