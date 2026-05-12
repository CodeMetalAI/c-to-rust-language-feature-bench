/* Test _Alignof returning minimum alignment for a type.  PR
   52023.  */
/* { dg-do run } */
/* { dg-options "-std=c11" } */

use std::mem;

fn main() {
    CHECK_ALIGN(_Bool);
    CHECK_ALIGN(char);
    CHECK_ALIGN(i8);
    CHECK_ALIGN(u8);
    CHECK_ALIGN(isize);
    CHECK_ALIGN(usize);
    CHECK_ALIGN(i16);
    CHECK_ALIGN(u16);
    CHECK_ALIGN(i32);
    CHECK_ALIGN(u32);
    CHECK_ALIGN(i64);
    CHECK_ALIGN(u64);
    CHECK_ALIGN(f32);
    CHECK_ALIGN(f64);
    CHECK_ALIGN(f80);
    CHECK_ALIGN(complex_f32);
    CHECK_ALIGN(complex_f64);
    CHECK_ALIGN(complex_f80);

}

macro_rules! CHECK_ALIGN {
    ($type:ty) => {{
        let x = [$type: _];
        if mem::align_of::<$type>() > mem::align_of::<x>()[0] {
            panic!();
        }
    }};
}