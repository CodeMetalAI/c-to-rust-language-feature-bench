fn main() {
    macro_rules! check_align {
        ($t:ty) => {{
            let x = [0u8, std::mem::size_of::<$t>() as u8];
            if std::mem::align_of::<$t>() > std::mem::align_of_val(&x[1]) {
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
    check_align!(f32);
    check_align!(f64);
    // Rust does not have built-in complex types, so these checks are omitted:
    // check_align!(_Complex float);
    // check_align!(_Complex double);
    // check_align!(_Complex long double);
}