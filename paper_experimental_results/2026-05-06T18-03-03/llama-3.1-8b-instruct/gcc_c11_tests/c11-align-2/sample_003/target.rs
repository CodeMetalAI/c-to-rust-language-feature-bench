#![feature(align_stack_check)]

use std::cmp::Ordering;

#[repr(align(16))]
struct MaxAlign;

#[repr(align(4))]
struct Int;

#[repr(align(8))]
struct Long;

#[repr(align(16))]
struct LongLong;

#[repr(align(16))]
struct Float;

#[repr(align(16))]
struct Double;

#[repr(align(16))]
struct ComplexLongDouble;

#[repr(C)]
struct S;

fn main() {
    let s1 = stringify!(alignas);
    let s2 = stringify!(alignof);
    let s3 = stringify!(__alignas_is_defined);
    let s4 = stringify!(__alignof_is_defined);

    assert_eq!(s1, "_Alignas");
    assert_eq!(s2, "_Alignof");
    assert_eq!(s3, "1");
    assert_eq!(s4, "1");
}