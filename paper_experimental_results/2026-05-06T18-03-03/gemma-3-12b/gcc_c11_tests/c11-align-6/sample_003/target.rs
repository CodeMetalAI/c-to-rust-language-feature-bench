#[cfg(target_os = "linux")]
#[inline(always)]
fn _Alignof<T>() -> usize {
    let layout = std::alloc::Layout::new::<T>();
    layout.align()
}

#[cfg(target_os = "windows")]
#[inline(always)]
fn _Alignof<T>() -> usize {
    let layout = std::alloc::Layout::new::<T>();
    layout.align()
}

#[cfg(target_os = "macos")]
#[inline(always)]
fn _Alignof<T>() -> usize {
    let layout = std::alloc::Layout::new::<T>();
    layout.align()
}

fn abort() {
    panic!("Assertion failed");
}

fn exit(code: i32) {
    std::process::exit(code);
}

macro_rules! CHECK_ALIGN {
    ($TYPE:ty) => {
        {
            struct {
                c: char,
                v: $TYPE,
            } x;
            if _Alignof::<$TYPE>() > _Alignof(x.v) {
                abort();
            }
        }
    };
}

fn main() {
    CHECK_ALIGN(_Bool);
    CHECK_ALIGN(char);
    CHECK_ALIGN(signed char);
    CHECK_ALIGN(unsigned char);
    CHECK_ALIGN(signed short);
    CHECK_ALIGN(unsigned short);
    CHECK_ALIGN(signed int);
    CHECK_ALIGN(unsigned int);
    CHECK_ALIGN(signed long);
    CHECK_ALIGN(unsigned long);
    CHECK_ALIGN(signed long long);
    CHECK_ALIGN(unsigned long long);
    CHECK_ALIGN(f32);
    CHECK_ALIGN(f64);
    CHECK_ALIGN(f128);
    CHECK_ALIGN(std::complex::Complex<f32>);
    CHECK_ALIGN(std::complex::Complex<f64>);
    CHECK_ALIGN(std::complex::Complex<f128>);
    exit(0);
}