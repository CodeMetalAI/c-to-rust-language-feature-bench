use std::mem;

macro_rules! CHECK_ALIGN {
    ($T:ty) => {
        {
            struct AlignTest {
                c: u8,
                v: $T,
            }
            let x = AlignTest { c: 0, v: unsafe { mem::zeroed() } };
            let align_of_type = mem::align_of::<$T>();
            let field_align = unsafe {
                let base = &x as *const _ as *const u8;
                let field_ptr = &x.v as *const _ as *const u8;
                let offset = field_ptr.offset_from(base) as usize;
                // Find the largest power of two that divides the offset
                offset & (!offset).wrapping_add(1)
            };
            if align_of_type > field_align {
                std::process::exit(1);
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
    CHECK_ALIGN!(isize);
    CHECK_ALIGN!(usize);
    CHECK_ALIGN!(f32);
    CHECK_ALIGN!(f64);
    // Rust doesn't have a direct long double equivalent; use f64 or f128 if available
    // We'll use f64 as the closest common equivalent
    CHECK_ALIGN!(f64);
    // Rust doesn't have built-in complex types; we'll use a tuple struct to mimic it
    CHECK_ALIGN!((f32, f32));
    CHECK_ALIGN!((f64, f64));
    CHECK_ALIGN!((f64, f64)); // For long double complex, also using f64 pair
    
    std::process::exit(0);
}