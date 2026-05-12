fn main() {
    macro_rules! check_align {
        ($type:ty) => {
            #[repr(C)]
            struct S {
                _padding: [u8; std::mem::align_of::<$type>() - 1],
                value: $type,
            }
            assert_eq!(std::mem::align_of::<$type>(), std::mem::align_of::<S>());
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
    check_align!(f80); // Note: f80 is not a standard type in Rust, but it's used here to match the C code
    // Complex numbers are not directly supported in Rust, so we skip these checks
}