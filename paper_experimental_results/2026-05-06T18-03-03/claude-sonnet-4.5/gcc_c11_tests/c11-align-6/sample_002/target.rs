use std::mem::{align_of, offset_of};
use std::process::exit;

macro_rules! CHECK_ALIGN {
    ($t:ty) => {{
        #[repr(C)]
        struct S {
            c: u8,
            v: $t,
        }
        if align_of::<$t>() > offset_of!(S, v) {
            panic!("Alignment check failed");
        }
    }};
}

fn main() {
    CHECK_ALIGN!(bool);
    CHECK_ALIGN!(i8);
    CHECK_ALIGN!(i8);
    CHECK_ALIGN!(u8);
    CHECK_ALIGN!(i16);
    CHECK_ALIGN!(u16);
    CHECK_ALIGN!(i32);
    CHECK_ALIGN!(u32);
    CHECK_ALIGN!(i64);
    CHECK_ALIGN!(u64);
    CHECK_ALIGN!(i64);
    CHECK_ALIGN!(u64);
    CHECK_ALIGN!(f32);
    CHECK_ALIGN!(f64);
    CHECK_ALIGN!(f64);
    CHECK_ALIGN!((f32, f32));
    CHECK_ALIGN!((f64, f64));
    CHECK_ALIGN!((f64, f64));
    exit(0);
}