fn main() {
    macro_rules! check_align {
        ($t:ty) => {{
            let x = (0u8, std::mem::MaybeUninit::<$t>::uninit());
            let alignment = std::mem::align_of::<$t>();
            let field_alignment = std::mem::align_of_val(&x.1);
            assert!(alignment <= field_alignment);
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
    // Rust does not support complex types natively, so we skip those checks.
}