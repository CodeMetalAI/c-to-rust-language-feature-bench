fn main() {
    CHECK_ALIGN(_Bool);
    CHECK_ALIGN(char);
    CHECK_ALIGN(i8);
    CHECK_ALIGN(u8);
    CHECK_ALIGN(i16);
    CHECK_ALIGN(u16);
    CHECK_ALIGN(i32);
    CHECK_ALIGN(u32);
    CHECK_ALIGN(i64);
    CHECK_ALIGN(u64);
    CHECK_ALIGN(i128);
    CHECK_ALIGN(u128);
    CHECK_ALIGN(f32);
    CHECK_ALIGN(f64);
    CHECK_ALIGN(f128);
    CHECK_ALIGN(std::complex::Complex<f32>);
    CHECK_ALIGN(std::complex::Complex<f64>);
    CHECK_ALIGN(std::complex::Complex<f128>);
    std::process::exit(0);
}

#[macro_export]
macro_rules! CHECK_ALIGN {
    ($TYPE) => {
        struct {
            c: char,
            v: $TYPE,
        } x;
        assert!(_alignof!($TYPE) <= core::mem::align_of!(x.v), "Alignment mismatch");
    };
}

#[macro_export]
macro_rules! _Bool {
    () => { bool };
}