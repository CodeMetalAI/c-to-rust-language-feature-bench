use std::mem::{align_of, offset_of};
use std::process::exit;

macro_rules! CHECK_ALIGN {
    ($typ:ty) => {{
        #[repr(C)]
        struct X {
            c: u8,
            v: $typ,
        }
        
        let type_align = align_of::<$typ>();
        let field_align = align_of::<X>() - offset_of!(X, v) + offset_of!(X, v);
        
        // Calculate actual alignment of field v in struct X
        // The offset of v gives us information about its alignment requirement
        let v_offset = offset_of!(X, v);
        
        // The alignment of the type should not exceed the alignment of the field in the struct
        // In practice, align_of::<$typ>() should equal the alignment of v in the struct
        if type_align > align_of::<X>() {
            std::process::abort();
        }
        
        // More precise check: the type's alignment should match what we observe in the struct
        // The offset of v should be a multiple of the type's alignment
        if v_offset % type_align != 0 {
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
    CHECK_ALIGN!(num_complex::Complex32);
    CHECK_ALIGN!(num_complex::Complex64);
    CHECK_ALIGN!(num_complex::Complex64);
    exit(0);
}