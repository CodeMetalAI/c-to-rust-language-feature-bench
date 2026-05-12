// Test Rust alignment support.  Test valid code using #[repr(align)].
// { dg-do run }
// { dg-options "-C" }

#[repr(align(align_of::<max_align_t>()))]
pub static c: [u8; 1];

#[repr(align(max_align_t))]
pub static s: [i16; 1];

#[repr(align(align_of::<i32>()))]
pub static i: i32;

#[repr(align(max_align_t))]
pub static l: i64;

#[repr(align(max_align_t))]
pub static ll: i64;

#[repr(align(max_align_t))]
pub static f: f32;

#[repr(align(max_align_t))]
pub static d: f64;

#[repr(align(max_align_t))]
pub static cld: complex128;

#[repr(align(align_of::<i32>()))]
pub static ca: [u8; 10];

#[repr(align((align_of::<max_align_t>() as usize) + 0))]
pub static x: i32;

#[repr(align(align_of::<max_align_t>()))]
pub enum E {
    E = align_of::<max_align_t>(),
}

#[repr(align(align_of::<max_align_t>()))]
pub static y: i32;

fn func() {
    #[repr(align(max_align_t))]
    let _auto_ll: i64;
}

// Valid, but useless.
#[repr(align(0))]
pub struct s;

fn main() {
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<E>());
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<s>());
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<s>());
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<s>());
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<s>());
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<s>());
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<s>());
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<s>());
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<s>());
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<s>());
    assert_eq!(std::mem::align_of::<max_align_t>(), std::mem::align_of::<s>());
}