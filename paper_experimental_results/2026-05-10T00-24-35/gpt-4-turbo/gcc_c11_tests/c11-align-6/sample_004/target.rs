fn main() {
    macro_rules! check_align {
        ($t:ty) => {
            {
                let x = (0u8, std::mem::MaybeUninit::<$t>::uninit());
                let alignment = std::mem::align_of::<$t>();
                let actual_alignment = std::mem::align_of_val(&x.1);
                if alignment > actual_alignment {
                    panic!("Alignment check failed");
                }
            }
        };
    }

    check_align!(bool);
    check_align!(char);
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
    check_align!(f64); // Rust doesn't support `long double` directly. `f64` is used as an approximation.
    // Not directly supported in Rust, using macros or external crates would be needed for Complex types:
    // check_align!(Complex<f32>);
    // check_align!(Complex<f64>);
    // check_align!(Complex<f64>); // As an approximation for `Complex long double`.
}