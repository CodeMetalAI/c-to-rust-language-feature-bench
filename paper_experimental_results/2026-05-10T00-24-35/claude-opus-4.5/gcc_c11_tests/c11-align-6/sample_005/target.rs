use std::mem::align_of;

macro_rules! check_align {
    ($type:ty) => {{
        #[repr(C)]
        struct AlignCheck {
            c: u8,
            v: $type,
        }
        
        // In Rust, align_of returns the alignment requirement for the type.
        // The alignment of a field within a struct will always be at least
        // the alignment of the type itself, so this check should always pass.
        // We're replicating the C behavior where _Alignof(TYPE) should not
        // exceed the actual alignment of the field in the struct.
        let type_align = align_of::<$type>();
        let field_offset = std::mem::offset_of!(AlignCheck, v);
        
        // The field's actual alignment in the struct can be inferred from its offset
        // For a properly aligned field, the offset should be a multiple of the type's alignment
        if field_offset % type_align != 0 {
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
    check_align!(f64); // Rust doesn't have long double, using f64
    // Rust doesn't have native complex types in std, but we can approximate
    // using tuples or arrays for the alignment check
    check_align!((f32, f32)); // Complex float approximation
    check_align!((f64, f64)); // Complex double approximation
    check_align!((f64, f64)); // Complex long double approximation
    
    std::process::exit(0);
}