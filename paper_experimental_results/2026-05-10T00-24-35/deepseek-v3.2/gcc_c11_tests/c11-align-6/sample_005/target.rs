use std::mem;

macro_rules! CHECK_ALIGN {
    ($TYPE:ty) => {
        {
            #[repr(C)]
            struct AlignTest {
                c: u8,
                v: $TYPE,
            }
            let x = AlignTest { c: 0, v: unsafe { mem::zeroed() } };
            let computed_align = mem::align_of::<$TYPE>();
            let struct_align = (&x.v as *const _ as usize) - (&x as *const _ as usize);
            if computed_align > struct_align {
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
    CHECK_ALIGN!(i128);
    CHECK_ALIGN!(u128);
    CHECK_ALIGN!(f32);
    CHECK_ALIGN!(f64);
    // Rust doesn't have a distinct 'long double' type; f64 is typically equivalent
    CHECK_ALIGN!(f64);
    // Rust doesn't have built-in complex types, but we can approximate with a tuple or struct
    // Using a simple repr(C) struct for complex
    #[repr(C)]
    struct ComplexF32 {
        re: f32,
        im: f32,
    }
    #[repr(C)]
    struct ComplexF64 {
        re: f64,
        im: f64,
    }
    CHECK_ALIGN!(ComplexF32);
    CHECK_ALIGN!(ComplexF64);
    CHECK_ALIGN!(ComplexF64);
    
    std::process::exit(0);
}