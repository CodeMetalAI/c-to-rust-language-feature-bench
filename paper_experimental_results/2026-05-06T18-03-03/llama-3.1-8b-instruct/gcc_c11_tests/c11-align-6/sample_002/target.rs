/* Test Rust align_of returning minimum alignment for a type.  PR
   52023.  */
/* { dg-do run } */
/* { dg-options "-std=c11" } */

use std::mem;

fn main() {
    #[allow(unused_macros)]
    macro_rules! check_align {
        ($type:ty) => {{
            let x = [$type::default(); 1];
            if mem::align_of::<$type>() > mem::align_of::<[u8; 1]>() {
                panic!("align_of returned incorrect value for $type");
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
    check_align!(f80);
    check_align!(complex_f32);
    check_align!(complex_f64);
    check_align!(complex_f80);
}

// Note: Rust does not have a direct equivalent to C's _Complex types.
// We use the complex_f32, complex_f64, and complex_f80 types from the
// Rust standard library instead.