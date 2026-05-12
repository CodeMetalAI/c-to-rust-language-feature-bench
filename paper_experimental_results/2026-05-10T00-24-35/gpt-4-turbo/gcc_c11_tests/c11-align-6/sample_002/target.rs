fn main() {
    macro_rules! check_align {
        ($type:ty) => {{
            let c: u8 = 0;
            let v: $type = Default::default();
            let base_ptr = &c as *const u8 as usize;
            let field_ptr = &v as *const $type as usize;
            let alignment = std::mem::align_of::<$type>();
            let offset = field_ptr.wrapping_sub(base_ptr);
            if alignment > offset {
                panic!("Alignment check failed");
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
    check_align!(f64); // Placeholder for long double, Rust doesn't have long double
    check_align!(num_complex::Complex<f32>);
    check_align!(num_complex::Complex<f64>);
    check_align!(num_complex::Complex<f64>); // Placeholder for Complex long double
}