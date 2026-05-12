fn main() {
    // CHECK_ALIGN(_Bool);
    check_align(std::bool);
    // CHECK_ALIGN(char);
    check_align(std::char);
    // CHECK_ALIGN(signed char);
    check_align(std::i8);
    // CHECK_ALIGN(unsigned char);
    check_align(std::u8);
    // CHECK_ALIGN(signed short);
    check_align(std::i16);
    // CHECK_ALIGN(unsigned short);
    check_align(std::u16);
    // CHECK_ALIGN(signed int);
    check_align(std::i32);
    // CHECK_ALIGN(unsigned int);
    check_align(std::u32);
    // CHECK_ALIGN(signed long);
    check_align(std::i64);
    // CHECK_ALIGN(unsigned long);
    check_align(std::u64);
    // CHECK_ALIGN(signed long long);
    check_align(std::i128);
    // CHECK_ALIGN(unsigned long long);
    check_align(std::u128);
    // CHECK_ALIGN(float);
    check_align(std::f32);
    // CHECK_ALIGN(double);
    check_align(std::f64);
    // CHECK_ALIGN(long double);
    check_align(std::f128);
    // CHECK_ALIGN(_Complex float);
    check_align(std::complex128::<f32>());
    // CHECK_ALIGN(_Complex double);
    check_align(std::complex128::<f64>());
    // CHECK_ALIGN(_Complex long double);
    check_align(std::complex128::<f128>());
}

fn check_align<T>(_: T) {
    struct X {
        c: u8,
        v: T,
    }
    assert!(std::mem::align_of::<T>() <= std::mem::align_of::<X>());
}