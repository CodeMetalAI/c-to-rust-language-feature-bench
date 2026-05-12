fn main() {
    macro_rules! check_align {
        ($type:ty) => {
            #[repr(C)]
            struct AlignCheck<T> {
                c: u8,
                v: T,
            }

            if std::mem::align_of::<$type>() > std::mem::align_of::<AlignCheck<$type>>() {
                std::process::exit(1);
            }
        };
    }

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
    check_align!(f128);
    check_align!(std::num::Complex<f32>);
    check_align!(std::num::Complex<f64>);
    check_align!(std::num::Complex<f128>);

    std::process::exit(0);
}