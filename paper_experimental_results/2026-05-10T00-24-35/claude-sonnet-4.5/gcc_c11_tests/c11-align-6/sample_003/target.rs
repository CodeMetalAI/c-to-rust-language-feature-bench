use std::mem::{align_of, offset_of};
use std::process::exit;

macro_rules! check_align {
    ($t:ty) => {{
        #[repr(C)]
        struct Wrapper {
            c: u8,
            v: $t,
        }
        
        let type_align = align_of::<$t>();
        let field_offset = offset_of!(Wrapper, v);
        
        // The field offset must be a multiple of the field's alignment
        // If type_align > actual alignment needed, this would fail
        if type_align > field_offset {
            std::process::abort();
        }
    }};
}

fn main() {
    check_align!(bool);
    check_align!(i8);
    check_align!(i8);
    check_align!(u8);
    check_align!(i16);
    check_align!(u16);
    check_align!(i32);
    check_align!(u32);
    check_align!(i64);
    check_align!(u64);
    check_align!(i64);
    check_align!(u64);
    check_align!(f32);
    check_align!(f64);
    check_align!(f64);
    check_align!((f32, f32));
    check_align!((f64, f64));
    check_align!((f64, f64));
    exit(0);
}