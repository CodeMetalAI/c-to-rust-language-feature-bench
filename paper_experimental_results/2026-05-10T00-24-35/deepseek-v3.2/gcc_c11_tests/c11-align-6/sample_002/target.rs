fn main() {
    macro_rules! check_align {
        ($type:ty) => {
            struct AlignTest {
                c: u8,
                v: $type,
            }
            let x = AlignTest { c: 0, v: <$type>::default() };
            if std::mem::align_of::<$type>() > std::mem::align_of_val(&x.v) {
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
    check_align!(f64); // long double typically maps to f64
    check_align!(num::Complex<f32>);
    check_align!(num::Complex<f64>);
    check_align!(num::Complex<f64>); // Complex<long double> typically maps to Complex<f64>
    
    std::process::exit(0);
}