fn main() {
    fn check_align<T>() {
        struct Wrapper<T> {
            c: u8,
            v: T,
        }
        let alignment = std::mem::align_of::<T>();
        let wrapper_size = std::mem::size_of::<Wrapper<T>>();
        if alignment > wrapper_size {
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
    check_align::<std::ffi::c_float>(); // Equivalent to long double in C
    // Note: Rust does not have direct equivalents for C's complex types.
    // Complex types typically require external crates or specific implementations.
    
    std::process::exit(0);
}