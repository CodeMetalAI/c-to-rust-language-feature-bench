fn main() {
    fn check_align<T>() {
        let x = (0u8, std::mem::MaybeUninit::<T>::uninit());
        let align_of_t = std::mem::align_of::<T>();
        let align_of_x_v = std::mem::align_of_val(&x.1);
        if align_of_t > align_of_x_v {
            std::process::abort();
        }
    }

    check_align::<bool>();
    check_align::<char>();
    check_align::<i8>();
    check_align::<u8>();
    check_align::<i16>();
    check_align::<u16>();
    check_align::<i32>();
    check_align::<u32>();
    check_align::<i64>();
    check_align::<u64>();
    check_align::<f32>();
    check_align::<f64>();
    check_align::<f128>(); // There is no direct representation for long double in Rust
    check_align::<(f32, f32)>();
    check_align::<(f64, f64)>();
    check_align::<(f128, f128)>();
    std::process::exit(0);
}