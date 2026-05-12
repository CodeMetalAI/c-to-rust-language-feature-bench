fn main() {
    macro_rules! check_align {
        ($type:ty) => {
            let x = [0 as u8, std::mem::align_of::<$type>()];
            assert!(std::mem::align_of::<$type>() <= std::mem::align_of::<[u8; 2]>());
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
    check_align!(f32);
    check_align!(f64);
    check_align!(f128);
    // Note: Rust does not have a direct equivalent to _Complex types.
    // The following lines are commented out as they would not compile.
    // check_align!(Complex<f32>);
    // check_align!(Complex<f64>);
    // check_align!(Complex<f128>);
}