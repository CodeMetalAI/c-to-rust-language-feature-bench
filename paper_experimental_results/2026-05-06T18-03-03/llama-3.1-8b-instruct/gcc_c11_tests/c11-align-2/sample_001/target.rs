// Test Rust alignment support.  Test valid code using std::align.

use std::cmp::Ordering;

#[repr(align(align_of::<max_align_t>()))]
struct C;

#[repr(align(max_align_t))]
struct CExtern;

#[repr(align(max_align_t))]
struct S;

#[repr(align(align_of::<max_align_t>()))]
struct I;

#[repr(align(max_align_t))]
struct L;

#[repr(align(max_align_t))]
struct LL;

#[repr(align(max_align_t))]
struct F;

#[repr(align(max_align_t))]
struct D;

#[repr(align(max_align_t))]
struct Cld;

#[repr(align(align_of::<max_align_t>()))]
struct Ca;

#[repr(align((align_of::<max_align_t>() as usize) + 0))]
struct X;

enum E {
    E = align_of::<max_align_t>(),
}

#[repr(align(E))]
struct Y;

fn func() {
    #[repr(align(max_align_t))]
    let _auto_ll: i64;
}

fn main() {
    assert_eq!(std::mem::align_of::<C>(), std::mem::align_of::<max_align_t>());
    assert_eq!(std::mem::align_of::<CExtern>(), std::mem::align_of::<max_align_t>());
    assert_eq!(std::mem::align_of::<S>(), std::mem::align_of::<max_align_t>());
    assert_eq!(std::mem::align_of::<I>(), std::mem::align_of::<max_align_t>());
    assert_eq!(std::mem::align_of::<L>(), std::mem::align_of::<max_align_t>());
    assert_eq!(std::mem::align_of::<LL>(), std::mem::align_of::<max_align_t>());
    assert_eq!(std::mem::align_of::<F>(), std::mem::align_of::<max_align_t>());
    assert_eq!(std::mem::align_of::<D>(), std::mem::align_of::<max_align_t>());
    assert_eq!(std::mem::align_of::<Cld>(), std::mem::align_of::<max_align_t>());
    assert_eq!(std::mem::align_of::<Ca>(), std::mem::align_of::<max_align_t>());
    assert_eq!(std::mem::align_of::<X>(), std::mem::align_of::<max_align_t>());
    assert_eq!(std::mem::align_of::<Y>(), std::mem::align_of::<max_align_t>());
}