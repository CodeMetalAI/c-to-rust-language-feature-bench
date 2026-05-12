fn main() {
    macro_rules! CHECK_ALIGN {
        ($TYPE:ty) => {
            #[repr(C)]
            struct AlignTest {
                c: u8,
                v: $TYPE,
            }
            let x = AlignTest { c: 0, v: unsafe { std::mem::zeroed() } };
            let align_of_type = std::mem::align_of::<$TYPE>();
            let field_align = {
                let base = &x as *const _ as *const u8;
                let field = &x.v as *const _ as *const u8;
                let offset = unsafe { field.offset_from(base) } as usize;
                if offset == 0 { 1 } else { offset.next_power_of_two() }
            };
            if align_of_type > field_align {
                std::process::abort();
            }
        };
    }

    CHECK_ALIGN!(bool);
    CHECK_ALIGN!(i8);
    CHECK_ALIGN!(u8);
    CHECK_ALIGN!(i16);
    CHECK_ALIGN!(u16);
    CHECK_ALIGN!(i32);
    CHECK_ALIGN!(u32);
    CHECK_ALIGN!(i64);
    CHECK_ALIGN!(u64);
    CHECK_ALIGN!(isize);
    CHECK_ALIGN!(usize);
    CHECK_ALIGN!(f32);
    CHECK_ALIGN!(f64);
    #[cfg(target_arch = "x86_64")]
    CHECK_ALIGN!(f64); // long double often same as f64 on x86_64
    #[cfg(not(target_arch = "x86_64"))]
    CHECK_ALIGN!(f64); // fallback
    CHECK_ALIGN!(num::Complex<f32>);
    CHECK_ALIGN!(num::Complex<f64>);
    #[cfg(target_arch = "x86_64")]
    CHECK_ALIGN!(num::Complex<f64>); // Complex long double
    #[cfg(not(target_arch = "x86_64"))]
    CHECK_ALIGN!(num::Complex<f64>); // fallback
}