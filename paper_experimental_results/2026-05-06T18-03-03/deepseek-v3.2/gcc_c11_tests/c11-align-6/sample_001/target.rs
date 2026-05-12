use std::mem;

macro_rules! CHECK_ALIGN {
    ($T:ty) => {
        {
            struct X {
                c: u8,
                v: $T,
            }
            let x = X { c: 0, v: unsafe { mem::zeroed() } };
            let align_of_type = mem::align_of::<$T>();
            let field_align = mem::align_of_val(&x.v);
            if align_of_type > field_align {
                std::process::abort();
            }
        }
    };
}

fn main() {
    CHECK_ALIGN!(bool);
    CHECK_ALIGN!(i8);
    CHECK_ALIGN!(u8);
    CHECK_ALIGN!(i16);
    CHECK_ALIGN!(u16);
    CHECK_ALIGN!(i32);
    CHECK_ALIGN!(u32);
    CHECK_ALIGN!(i64);
    CHECK_ALIGN!(u64);
    CHECK_ALIGN!(i128);
    CHECK_ALIGN!(u128);
    CHECK_ALIGN!(f32);
    CHECK_ALIGN!(f64);
    #[cfg(target_arch = "x86_64")]
    CHECK_ALIGN!(f64); // long double is typically f64 on x86_64
    #[cfg(target_arch = "x86")]
    CHECK_ALIGN!(f64); // long double is typically f64 on x86
    #[cfg(target_arch = "aarch64")]
    CHECK_ALIGN!(f64); // long double is typically f64 on aarch64
    // Complex types are not standard in Rust; we approximate with tuples.
    CHECK_ALIGN!((f32, f32));
    CHECK_ALIGN!((f64, f64));
    #[cfg(target_arch = "x86_64")]
    CHECK_ALIGN!((f64, f64));
    #[cfg(target_arch = "x86")]
    CHECK_ALIGN!((f64, f64));
    #[cfg(target_arch = "aarch64")]
    CHECK_ALIGN!((f64, f64));
    std::process::exit(0);
}