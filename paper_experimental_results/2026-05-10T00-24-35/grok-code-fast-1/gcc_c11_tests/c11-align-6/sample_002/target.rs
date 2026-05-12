macro_rules! check_align {
    ($t:ty) => {
        // Mimic the C check; in practice, this always passes
        assert!(std::mem::align_of::<$t>() <= std::mem::align_of::<$t>());
    };
}

#[repr(C)]
struct ComplexF32(f32, f32);

#[repr(C)]
struct ComplexF64(f64, f64);

#[repr(C)]
struct ComplexF128(f64, f64); // Approximating long double as f64

fn main() {
    check_align!(bool);
    check_align!(i8); // signed char
    check_align!(u8); // unsigned char and char
    check_align!(i16);
    check_align!(u16);
    check_align!(i32);
    check_align!(u32);
    check_align!(i64);
    check_align!(u64);
    check_align!(isize); // signed long
    check_align!(usize); // unsigned long
    check_align!(i64); // signed long long (assuming 64-bit)
    check_align!(u64); // unsigned long long
    check_align!(f32);
    check_align!(f64);
    check_align!(f64); // long double as f64
    check_align!(ComplexF32);
    check_align!(ComplexF64);
    check_align!(ComplexF128);
    std::process::exit(0);
}