use std::alloc::Layout;
use std::process;

macro_rules! check_align {
    ($ty:ty) => {
        #[repr(C)]
        struct AlignCheck<T> {
            c: u8,
            v: T,
        }

        let align_of_ty = Layout::new::<$ty>().align();
        let align_of_v = Layout::new::<AlignCheck<$ty>>().align();
        if align_of_ty > align_of_v {
            process::exit(1);
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
    check_align!(i128);
    check_align!(u128);
    check_align!(f32);
    check_align!(f64);
    check_align!(f64); // Rust doesn't have long double, so we use f64
    check_align!(std::num::Complex<f32>);
    check_align!(std::num::Complex<f64>);
    check_align!(std::num::Complex<f64>); // Rust doesn't have long double, so we use f64
    process::exit(0);
}