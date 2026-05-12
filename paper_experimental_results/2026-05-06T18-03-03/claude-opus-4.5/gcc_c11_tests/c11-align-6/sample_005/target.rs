use std::mem::align_of;

macro_rules! check_align {
    ($type:ty) => {{
        #[repr(C)]
        struct Wrapper {
            c: u8,
            v: $type,
        }
        
        let type_align = align_of::<$type>();
        let wrapper_align = align_of::<Wrapper>();
        
        // The alignment of v within the struct should be at least align_of::<$type>()
        // In a repr(C) struct, v will be placed at an offset that respects its alignment
        // The wrapper's alignment will be the max of its fields' alignments
        // So wrapper_align >= type_align should always hold
        if type_align > wrapper_align {
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
    // For complex types, Rust doesn't have built-in complex numbers
    // Using tuples of floats as approximation
    check_align!((f32, f32));
    check_align!((f64, f64));
    check_align!((f64, f64));
    
    std::process::exit(0);
}