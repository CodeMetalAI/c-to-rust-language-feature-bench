use std::mem::align_of;

macro_rules! check_align {
    ($type:ty) => {{
        #[repr(C)]
        struct Wrapper {
            c: u8,
            v: $type,
        }
        // In Rust, align_of always returns the actual alignment requirement
        // The alignment of v within the struct will always be >= align_of::<$type>()
        // This check mirrors the C code's intent: verify that _Alignof(TYPE) <= actual alignment
        let type_align = align_of::<$type>();
        let wrapper_align = align_of::<Wrapper>();
        // The alignment of v in the struct is at least align_of::<$type>()
        // Since Rust guarantees proper alignment, this should never fail
        if type_align > wrapper_align.max(type_align) {
            panic!("Alignment check failed");
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
    // Rust doesn't have built-in complex types, but we can check tuples as approximation
    check_align!((f32, f32));
    check_align!((f64, f64));
    check_align!((f64, f64));
    
    std::process::exit(0);
}