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
            let align_of_type = mem::align_of::<$TYPE>();
            let actual_align = (&x.v as *const _ as usize) - (&x as *const _ as usize);
            if align_of_type > actual_align {
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
    CHECK_ALIGN!(f64); // long double typically maps to f64 in Rust
    CHECK_ALIGN!(num::Complex<f32>);
    CHECK_ALIGN!(num::Complex<f64>);
    CHECK_ALIGN!(num::Complex<f64>); // Complex long double typically maps to Complex<f64>
    
    std::process::exit(0);
}