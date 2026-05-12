#[cfg(target_has_feature = "cmse-nonsecure")]
#[allow(dead_code)]
fn abort() {
    panic!("abort called");
}

#[allow(dead_code)]
fn exit(status: i32) {
    std::process::exit(status);
}

macro_rules! CHECK_ALIGN {
    ($TYPE:ty) => {
        {
            #[repr(packed)]
            struct {
                c: u8,
                v: $TYPE,
            } x;
            assert!(std::mem::align_of::<$TYPE>() <= std::mem::align_of!(x.v));
        }
    };
}

fn main() {
    CHECK_ALIGN(_Bool);
    CHECK_ALIGN(i8);
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
    exit(0);
}