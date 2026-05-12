fn check_align<T>() {
    struct AlignCheck<T> {
        c: u8,
        v: T,
    }

    let align_of_t = std::mem::align_of::<T>();
    let align_of_v = std::mem::align_of_val(&AlignCheck::<T> { c: 0, v: unsafe { std::mem::zeroed() } }.v);

    if align_of_t > align_of_v {
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
    check_align::<f128>();
    check_align::<std::num::Complex<f32>>();
    check_align::<std::num::Complex<f64>>();
    check_align::<std::num::Complex<f128>>();

    std::process::exit(0);
}