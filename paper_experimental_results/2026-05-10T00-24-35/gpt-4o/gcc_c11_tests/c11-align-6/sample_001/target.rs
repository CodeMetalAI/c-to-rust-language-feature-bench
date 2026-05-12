fn check_align<T>() {
    #[repr(C)]
    struct Aligned {
        c: u8,
        v: T,
    }

    if std::mem::align_of::<T>() > std::mem::align_of::<Aligned>() {
        std::process::abort();
    }
}

fn main() {
    check_align::<bool>();
    check_align::<i8>();
    check_align::<u8>();
    check_align::<i16>();
    check_align::<u16>();
    check_align::<i32>();
    check_align::<u32>();
    check_align::<i64>();
    check_align::<u64>();
    check_align::<i128>();
    check_align::<u128>();
    check_align::<f32>();
    check_align::<f64>();
    // Note: Rust does not directly support complex numbers like C's `_Complex` types.
    // If needed, use an external crate for complex numbers, such as `num-complex`.
    // check_align::<num_complex::Complex<f32>>();
    // check_align::<num_complex::Complex<f64>>();
    // check_align::<num_complex::Complex<f128>>();
    
    std::process::exit(0);
}