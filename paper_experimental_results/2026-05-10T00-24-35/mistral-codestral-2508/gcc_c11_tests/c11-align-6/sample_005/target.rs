fn check_align<T>() {
    #[repr(C)]
    struct AlignCheck<T> {
        c: u8,
        v: T,
    }

    if std::mem::align_of::<T>() > std::mem::align_of_val(&AlignCheck::<T> { c: 0, v: unsafe { std::mem::zeroed() } }.v) {
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
    check_align::<f64>(); // Rust doesn't have long double, using f64 as a substitute
    check_align::<std::num::Complex<f32>>();
    check_align::<std::num::Complex<f64>>();
    check_align::<std::num::Complex<f64>>(); // Rust doesn't have long double complex, using f64 as a substitute
    std::process::exit(0);
}