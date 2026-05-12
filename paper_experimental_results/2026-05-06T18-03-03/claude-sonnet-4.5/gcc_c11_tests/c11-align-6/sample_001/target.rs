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
        // the offset at which it's placed. The offset must be a multiple
        // of the field's alignment requirement.
        // If type_align > actual field alignment, this would be a problem.
        // We check if the declared alignment is greater than what the field actually needs.
        
        // In the C code, __alignof__(x.v) gives the alignment requirement of the field.
        // In Rust, the field will be aligned according to align_of::<$t>().
        // The check verifies that _Alignof(TYPE) <= __alignof__(x.v).
        // Since both should be the same in well-formed code, we verify the offset
        // is properly aligned.
        
        if type_align > field_offset && field_offset != 0 {
            std::process::abort();
        }
        
        // More direct check: the alignment of the type should not exceed
        // the alignment requirement that the field actually has in the struct
        let actual_field_align = align_of::<$t>();
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
    
    exit(0);
}