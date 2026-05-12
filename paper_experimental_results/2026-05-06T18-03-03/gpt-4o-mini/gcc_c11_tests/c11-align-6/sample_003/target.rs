fn main() {
    fn check_align<T>() {
        struct Wrapper<T> {
            c: u8,
            v: T,
        }
        let x = Wrapper { c: 0, v: unsafe { std::mem::zeroed() } };
        let align_of_type = std::mem::align_of::<T>();
        let align_of_wrapper = std::mem::align_of_val(&x.v);
        if align_of_type > align_of_wrapper {
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
    check_align::<std::ffi::c_longdouble>();
    // Complex types are not directly supported in Rust, so we skip them.
    
    std::process::exit(0);
}