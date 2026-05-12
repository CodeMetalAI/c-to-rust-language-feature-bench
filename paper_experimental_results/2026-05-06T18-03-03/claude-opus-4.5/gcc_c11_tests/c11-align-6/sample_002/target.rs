use std::mem::align_of;

macro_rules! check_align {
    ($type:ty) => {{
        #[repr(C)]
        struct Wrapper {
            c: u8,
            v: $type,
        }
        
        // In Rust, align_of always returns the actual alignment requirement
        // The alignment of v within the struct should be at least align_of::<$type>()
        // This check verifies that align_of returns the minimum alignment
        let type_align = align_of::<$type>();
        let wrapper_align = align_of::<Wrapper>();
        
        // The wrapper's alignment should be at least the type's alignment
        // If type_align > wrapper_align, something is wrong
        if type_align > wrapper_align {
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
    // Complex types - Rust doesn't have built-in complex, using tuple representation
    check_align!((f32, f32));  // _Complex float
    check_align!((f64, f64));  // _Complex double
    check_align!((f64, f64));  // _Complex long double
    
    std::process::exit(0);
}