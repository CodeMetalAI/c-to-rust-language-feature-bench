use std::mem::align_of;

macro_rules! check_align {
    ($type:ty) => {{
        #[repr(C)]
        struct AlignCheck {
            c: u8,
            v: $type,
        }
        
        // In Rust, align_of always returns the alignment requirement of the type,
        // which is consistent. The C test checks that _Alignof(TYPE) <= __alignof__(x.v)
        // In safe Rust, the alignment of a field is always at least align_of::<T>(),
        // so this check would always pass. We replicate the structure of the test.
        let type_align = align_of::<$type>();
        let struct_size = std::mem::size_of::<AlignCheck>();
        let char_size = std::mem::size_of::<u8>();
        let v_size = std::mem::size_of::<$type>();
        
        // Calculate the offset of v in the struct, which tells us the effective alignment
        // offset = align_up(char_size, align_of::<$type>())
        let v_offset = struct_size - v_size;
        
        // The actual alignment used for v is at least type_align
        // In Rust this is always true by construction
        if type_align > v_offset && v_offset != 0 {
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
    check_align!(f64);  // Rust doesn't have long double, using f64
    // Rust doesn't have native complex types in std, so we'll use tuples as approximation
    check_align!((f32, f32));
    check_align!((f64, f64));
    check_align!((f64, f64));
    
    std::process::exit(0);
}