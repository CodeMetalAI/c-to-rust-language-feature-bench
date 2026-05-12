use std::mem::{align_of, size_of};
use std::process::exit;

macro_rules! CHECK_ALIGN {
    ($ty:ty) => {{
        #[repr(C)]
        struct Wrapper {
            c: u8,
            v: $ty,
        }
        
        let type_align = align_of::<$ty>();
        let field_offset = size_of::<u8>().next_multiple_of(align_of::<$ty>());
        let field_align = align_of::<$ty>();
        
        if type_align > field_align {
            std::process::abort();
        }
    }};
}

fn main() {
    CHECK_ALIGN!(bool);
    CHECK_ALIGN!(i8);
    CHECK_ALIGN!(i8);
    CHECK_ALIGN!(u8);
    CHECK_ALIGN!(i16);
    CHECK_ALIGN!(u16);
    CHECK_ALIGN!(i32);
    CHECK_ALIGN!(u32);
    CHECK_ALIGN!(i64);
    CHECK_ALIGN!(u64);
    CHECK_ALIGN!(i64);
    CHECK_ALIGN!(u64);
    CHECK_ALIGN!(f32);
    CHECK_ALIGN!(f64);
    CHECK_ALIGN!(f64);
    CHECK_ALIGN!((f32, f32));
    CHECK_ALIGN!((f64, f64));
    CHECK_ALIGN!((f64, f64));
    exit(0);
}