use std::mem;
use std::process;

macro_rules! check_align {
    ($t:ty) => {{
        #[repr(C)]
        struct Check {
            c: u8,
            v: $t,
        }
        let x = Check {
            c: 0,
            v: Default::default(),
        };
        if mem::align_of::<$t>() > mem::align_of_val(&x.v) {
            process::abort();
        }
    }};
}

fn main() {
    check_align!(bool);
    check_align!(i8);
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
    check_align!((f32, f32));
    check_align!((f64, f64));
    check_align!((f64, f64));
    process::exit(0);
}