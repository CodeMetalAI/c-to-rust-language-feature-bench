fn main() {
    macro_rules! check_align {
        ($type:ty) => {{
            struct AlignCheck {
                _pad: [u8; 0],
                value: $type,
            }
            let align = std::mem::align_of::<$type>();
            let actual_align = std::mem::align_of::<AlignCheck>();
            assert!(align <= actual_align);
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
    check_align!(f80); // Note: Rust doesn't have a direct equivalent to long double
    // check_align!(Complex f32); // Note: Rust doesn't have a built-in complex number type
    // check_align!(Complex f64); // Note: Rust doesn't have a built-in complex number type
    // check_align!(Complex f80); // Note: Rust doesn't have a built-in complex number type
}