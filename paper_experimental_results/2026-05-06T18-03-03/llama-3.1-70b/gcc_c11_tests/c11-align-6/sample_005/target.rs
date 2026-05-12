fn main() {
    macro_rules! check_align {
        ($type:ty) => {
            let x = std::mem::align_of::<$type>();
            let y = std::mem::align_of::<(char, $type)>();
            if x > y {
                std::process::abort();
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
    check_align!(f128);
    // Note: Rust does not have a direct equivalent to _Complex types.
    // The following lines are commented out as they would not compile.
    // check_align!(std::complex::Complex32);
    // check_align!(std::complex::Complex64);
    // check_align!(std::complex::Complex128);
    std::process::exit(0);
}