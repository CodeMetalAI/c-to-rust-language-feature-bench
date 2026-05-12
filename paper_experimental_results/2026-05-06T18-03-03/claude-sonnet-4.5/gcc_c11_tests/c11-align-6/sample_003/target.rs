use std::mem::{align_of, offset_of};
use std::process::exit;

macro_rules! CHECK_ALIGN {
    ($t:ty) => {{
        #[repr(C)]
        struct Wrapper {
            c: u8,
            v: $t,
        }
        
        let type_align = align_of::<$t>();
        let field_offset = offset_of!(Wrapper, v);
        
        // The actual alignment of field v in the struct is determined by
        // the offset at which it's placed. If type_align > actual alignment,
        // that would be a problem (abort condition in original C code).
        // In practice, field_offset will be a multiple of the field's alignment.
        // We check if the declared alignment exceeds what the struct layout provides.
        if type_align > field_offset {
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