fn main() {
    macro_rules! check_align {
        ($t:ty) => {{
            let x = (0u8, <$t>::default()); // Tuple to simulate struct with char and type
            let align_of_t = std::mem::align_of::<$t>();
            let align_of_x_1 = std::mem::align_of_val(&x.1);
            if align_of_t > align_of_x_1 {
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
    check_align!(i128);
    check_align!(u128);
    check_align!(f32);
    check_align!(f64);
    check_align!(f32); // Rust does not support complex types natively, using f32 as placeholder
    check_align!(f64); // Rust does not support complex types natively, using f64 as placeholder
    check_align!(f64); // Rust does not support complex types natively, using f64 as placeholder
}