fn main() {
    fn check_align<T>() {
        struct Wrapper {
            c: u8,
            v: T,
        };
        
        let align_of_t = std::mem::align_of::<T>();
        let align_of_v = std::mem::align_of::<Wrapper>();
        
        if align_of_t > align_of_v {
            std::process::abort();
        }
    }
    
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
    // Rust doesn't have long double or complex types directly,
    // but we can provide align checks for regular doubles and use external crates for complex numbers if needed
    
    std::process::exit(0);
}