use std::mem;
use std::process;

macro_rules! check_align {
    ($t:ty) => {
        if mem::align_of::<$t>() > mem::align_of::<$t>() {
            process::abort();
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
    check_align!(i64);
    check_align!(u64);
    check_align!(f32);
    check_align!(f64);
    process::exit(0);
}