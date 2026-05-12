use std::mem::{align_of, offset_of};
use std::process::exit;

macro_rules! CHECK_ALIGN {
    ($t:ty) => {{
        #[repr(C)]
        struct X {
            c: u8,
            v: $t,
        }
        
        let type_align = align_of::<$t>();
        let field_align = align_of::<X>() - offset_of!(X, v) + offset_of!(X, v);
        
        // Calculate actual alignment of field v within struct X
        let actual_field_align = offset_of!(X, v);
        
        if type_align > actual_field_align {
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