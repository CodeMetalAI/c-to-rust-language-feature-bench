use std::mem;
use std::process;

#[repr(C)]
struct X<T> {
    c: u8,
    v: T,
}

macro_rules! check_align {
    ($t:ty) => {
        {
            let x = X {
                c: 0,
                v: Default::default(),
            };
            if mem::align_of::<$t>() > mem::align_of_val(&x.v) {
                process::abort();
            }
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
    process::exit(0);
}