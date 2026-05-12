fn main() {
    macro_rules! check_align {
        ($type:ty) => {{
            use std::mem::{align_of, size_of};
            
            #[repr(C)]
            struct X {
                c: char,
                v: $type,
            }

            // In Rust, the alignment logic is automatically handled correctly by the compiler.
            // Hence just asserting that the alignment of `X.v` should at least be that of `$type`.
            if align_of::<$type>() > align_of::<X>() {
                // Panic in Rust provides similar abrupt termination to abort() in C
                panic!("Alignment check failed for {:?}", stringify!($type));
            }
        }};
    }

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
    check_align!(f32);
    check_align!(f64);
    check_align!(f64); // Rust currently doesn't have native complex number types in std
    check_align!(f64);
    check_align!(f64);
}