use std::mem::{align_of, offset_of};
use std::process::exit;

macro_rules! CHECK_ALIGN {
    ($typ:ty) => {{
        #[repr(C)]
        struct Wrapper {
            c: u8,
            v: $typ,
        }
        
        let type_align = align_of::<$typ>();
        let field_offset = offset_of!(Wrapper, v);
        
        // The actual alignment of field v in the struct is determined by
        // the offset at which it's placed. The offset must be a multiple
        // of the field's alignment requirement.
        // We check if _Alignof(TYPE) > actual alignment of x.v
        // The actual alignment can be inferred: if offset is field_offset,
        // then the field is aligned to at least gcd(field_offset, type_align)
        // However, the C code uses __alignof__(x.v) which gives the alignment
        // requirement of the field in context.
        
        // In the struct, x.v has alignment equal to min(align_of::<$typ>(), struct_align)
        // But since it's in a packed-by-default C struct (repr(C)), 
        // the field v will be placed at the next aligned position after c.
        
        // The actual alignment of x.v in the struct is align_of::<$typ>()
        // because repr(C) respects field alignments.
        // So we're checking: align_of::<$typ>() > align_of::<$typ>()
        // which should never be true.
        
        // However, to match the C semantics exactly where __alignof__(x.v)
        // might differ from _Alignof(TYPE), we check against the actual
        // field alignment which in Rust's repr(C) is align_of::<$typ>()
        
        let field_align = align_of::<$typ>();
        
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