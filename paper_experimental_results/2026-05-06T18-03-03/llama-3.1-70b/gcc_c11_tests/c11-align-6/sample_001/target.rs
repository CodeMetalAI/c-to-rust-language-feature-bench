fn main() {
    macro_rules! check_align {
        ($type:ty) => {
            let x = [0 as u8, 0 as $type];
            let align = std::mem::align_of::<$type>();
            let actual_align = std::mem::align_of_val(&x[1]);
            if align > actual_align {
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
    check_align!(f64); // Rust does not have a direct equivalent to long double
    // Rust does not have a direct equivalent to _Complex types
    std::process::exit(0);
}