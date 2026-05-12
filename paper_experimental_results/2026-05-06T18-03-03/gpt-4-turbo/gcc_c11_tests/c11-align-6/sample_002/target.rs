fn main() {
    macro_rules! check_align {
        ($t:ty) => {{
            let x = (0u8, <$t>::default()); // Tuple to simulate struct with padding
            let align_of_t = std::mem::align_of::<$t>();
            let align_of_field = std::mem::align_of_val(&x.1);
            assert!(align_of_t <= align_of_field);
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
    // Rust does not have built-in complex types, so we skip those checks.
}