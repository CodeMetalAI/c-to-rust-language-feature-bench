use std::mem::align_of;

macro_rules! check_align {
    ($type:ty) => {{
        #[repr(C)]
        struct Wrapper {
            c: u8,
            v: $type,
        }
        // In Rust, align_of returns the alignment requirement for a type.
        // The alignment of a field within a struct is at least align_of for that type.
        // This check verifies that align_of(TYPE) <= actual alignment in struct,
        // which is always true in Rust by design, but we replicate the test logic.
        let type_align = align_of::<$type>();
        let wrapper_align = align_of::<Wrapper>();
        // The field v in Wrapper will be aligned to at least align_of::<$type>()
        // We can verify this by checking that the struct's alignment accommodates it
        if type_align > wrapper_align {
            panic!("Alignment check failed");
        }
    }};
}

fn main() {
    check_align!(bool);
    check_align!(i8);
    check_align!(i8);  // signed char
    check_align!(u8);  // unsigned char
    check_align!(i16); // signed short
    check_align!(u16); // unsigned short
    check_align!(i32); // signed int
    check_align!(u32); // unsigned int
    check_align!(i64); // signed long (on 64-bit systems)
    check_align!(u64); // unsigned long
    check_align!(i64); // signed long long
    check_align!(u64); // unsigned long long
    check_align!(f32); // float
    check_align!(f64); // double
    // Note: Rust doesn't have a direct equivalent to long double or _Complex types
    // Using f64 as the closest approximation for long double
    check_align!(f64); // long double approximation
    // For complex types, we use tuples of floats as approximation
    check_align!((f32, f32)); // _Complex float approximation
    check_align!((f64, f64)); // _Complex double approximation
    check_align!((f64, f64)); // _Complex long double approximation
    
    std::process::exit(0);
}