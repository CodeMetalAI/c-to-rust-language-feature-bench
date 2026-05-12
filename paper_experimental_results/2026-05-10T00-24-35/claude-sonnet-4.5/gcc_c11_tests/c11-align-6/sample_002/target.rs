use std::mem::{align_of, offset_of};
use std::process::exit;

macro_rules! CHECK_ALIGN {
    ($type:ty) => {{
        #[repr(C)]
        struct Wrapper {
            c: u8,
            v: $type,
        }
        
        let type_align = align_of::<$type>();
        let field_offset = offset_of!(Wrapper, v);
        
        // The actual alignment of field v in the struct is determined by the offset
        // In a C struct with char followed by TYPE, the offset of v gives us alignment info
        // If _Alignof(TYPE) > actual alignment of v in struct, we should abort
        // The offset must be a multiple of the field's alignment
        // So we check if the declared alignment is consistent with the offset
        
        // Actually, we need to check the alignment of the field itself
        // The alignment of v in the struct should be at least type_align
        // but due to packing, it might be less in practice
        // We're checking if type_align > the actual alignment observed
        
        // The offset of v must be aligned to align_of::<$type>()
        // If type_align is properly respected, offset should be divisible by it
        // The test checks if _Alignof(TYPE) > __alignof__(x.v)
        // In Rust, align_of gives us the alignment requirement
        
        // For this test, we check that the alignment requirement doesn't exceed
        // what's actually used in the struct layout
        if type_align > field_offset && field_offset != 0 {
            std::process::abort();
        }
        
        // More accurately: the field offset must be a multiple of the type's alignment
        // If it's not, then the actual alignment is less than declared
        if field_offset % type_align != 0 {
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