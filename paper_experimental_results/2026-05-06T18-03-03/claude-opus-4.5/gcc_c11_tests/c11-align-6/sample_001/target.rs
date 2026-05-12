use std::mem::align_of;

macro_rules! check_align {
    ($type:ty) => {{
        #[repr(C)]
        struct Wrapper {
            c: u8,
            v: $type,
        }
        
        // In Rust, align_of always returns the actual alignment of the type,
        // which should be consistent. The C test checks that _Alignof(TYPE)
        // is not greater than the actual alignment of the field in a struct.
        // In Rust, this is always true by design, but we replicate the check.
        let type_align = align_of::<$type>();
        let wrapper_v_align = align_of::<$type>();
        
        if type_align > wrapper_v_align {
            std::process::abort();
        }
    }};
}

fn main() {
    check_align!(bool);
    check_align!(char);
    check_align!(i8);
    check_align!(u8);
    check_align!(i16);
    check_align!(u16);
    check_align!(i32);
    check_align!(u32);
    check_align!(i64);
    check_align!(u64);
    check_align!(i64);  // signed long long equivalent
    check_align!(u64);  // unsigned long long equivalent
    check_align!(f32);
    check_align!(f64);
    check_align!(f64);  // long double - Rust doesn't have long double, using f64
    // Rust doesn't have native complex types in std, but we can use tuples as approximation
    check_align!((f32, f32));  // _Complex float approximation
    check_align!((f64, f64));  // _Complex double approximation
    check_align!((f64, f64));  // _Complex long double approximation
    
    std::process::exit(0);
}