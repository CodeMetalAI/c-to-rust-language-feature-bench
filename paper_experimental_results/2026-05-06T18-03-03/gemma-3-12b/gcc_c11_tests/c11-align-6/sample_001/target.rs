#[cfg(target_os = "linux")]
#[inline(always)]
fn alignof<T>() -> usize {
    core::mem::align_of::<T>()
}

#[cfg(target_os = "windows")]
#[inline(always)]
fn alignof<T>() -> usize {
    core::mem::align_of::<T>()
}

#[cfg(target_os = "macos")]
#[inline(always)]
fn alignof<T>() -> usize {
    core::mem::align_of::<T>()
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
#[inline(always)]
fn alignof<T>() -> usize {
    core::mem::align_of::<T>()
}

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
}

#[macro_export]
macro_rules! CHECK_ALIGN {
    ($TYPE:ty) => {
        struct {
            c: char,
            v: $TYPE,
        } x;
        if alignof::$TYPE() > alignof!(x.v) {
            panic!("Alignment mismatch");
        }
    };
}