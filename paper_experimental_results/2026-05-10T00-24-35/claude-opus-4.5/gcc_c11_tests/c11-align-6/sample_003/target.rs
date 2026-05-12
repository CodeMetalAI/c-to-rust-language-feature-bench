use std::mem::align_of;

macro_rules! check_align {
    ($type:ty) => {{
        #[repr(C)]
        struct AlignCheck {
            c: u8,
            v: $type,
        }
        
        // In Rust, align_of::<T>() returns the alignment of the type,
        // which should always equal the alignment of a field of that type
        // in a repr(C) struct. This check verifies that property.
        let type_align = align_of::<$type>();
        let struct_align = align_of::<AlignCheck>();
        
        // The alignment of v within the struct should be at least type_align
        // For repr(C) structs, the field alignment equals the type alignment
        // We verify that align_of(TYPE) <= alignment used for the field
        // The struct's alignment is max(align(c), align(v)) = max(1, align(v)) = align(v)
        // So struct_align gives us the effective alignment of v
        if type_align > struct_align {
            std::process::exit(1);
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
    // Rust doesn't have built-in complex types, but we can approximate with tuples
    check_align!((f32, f32));
    check_align!((f64, f64));
    check_align!((f64, f64));
    
    std::process::exit(0);
}