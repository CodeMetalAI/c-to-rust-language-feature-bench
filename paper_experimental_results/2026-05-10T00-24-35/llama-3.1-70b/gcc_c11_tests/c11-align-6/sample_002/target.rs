fn main() {
    macro_rules! check_align {
        ($ty:ty) => {{
            struct AlignCheck {
                _padding: [u8; 1],
                value: $ty,
            }

            let align_of_ty = std::mem::align_of::<$ty>();
            let align_of_check = std::mem::align_of::<AlignCheck>();

            if align_of_ty > align_of_check {
                panic!();
            }
        }};
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
    // Note: Rust does not have a direct equivalent to _Complex long double.
    // The following is a rough approximation.
    check_align!(f80);

    std::process::exit(0);
}