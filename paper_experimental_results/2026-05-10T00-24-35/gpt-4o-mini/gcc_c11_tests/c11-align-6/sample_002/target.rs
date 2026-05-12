fn main() {
    fn check_align<T>() {
        let _x = (0u8, std::mem::align_of::<T>());
        if std::mem::align_of::<T>() > std::mem::align_of_val(&_x.1) {
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
    
    std::process::exit(0);
}