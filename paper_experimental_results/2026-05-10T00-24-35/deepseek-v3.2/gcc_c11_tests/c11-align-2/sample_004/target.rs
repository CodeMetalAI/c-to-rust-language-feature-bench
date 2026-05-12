use std::mem::{align_of, align_of_val};

// Define max_align_t equivalent
#[repr(C)]
union MaxAlign {
    _i: std::mem::MaybeUninit<i64>,
    _f: std::mem::MaybeUninit<f64>,
    _d: std::mem::MaybeUninit<[u8; 32]>,
}

const MAX_ALIGN: usize = align_of::<MaxAlign>();

// Variables with alignments
#[repr(align(MAX_ALIGN))]
static mut C: [u8; 1] = [0];

#[repr(align(MAX_ALIGN))]
static mut S: i16 = 0;

#[repr(align(4))]
static mut I: i32 = 0;

#[repr(align(MAX_ALIGN))]
static mut L: i64 = 0;

#[repr(align(MAX_ALIGN))]
static mut LL: i64 = 0;

#[repr(align(MAX_ALIGN))]
static mut F: f32 = 0.0;

#[repr(align(MAX_ALIGN))]
static mut D: f64 = 0.0;

// Complex long double approximation
#[repr(C)]
struct ComplexLongDouble {
    real: f64,
    imag: f64,
}

#[repr(align(MAX_ALIGN))]
static mut CLD: ComplexLongDouble = ComplexLongDouble { real: 0.0, imag: 0.0 };

#[repr(align(1), align(4))]
static mut CA: [u8; 10] = [0; 10];

#[repr(align(MAX_ALIGN))]
static mut X: i32 = 0;

enum E {
    E = MAX_ALIGN,
}

#[repr(align(E::E as usize))]
static mut Y: i32 = 0;

// Struct with alignment warning equivalent
#[repr(align(1))]
struct S;

// Macro stringification simulation
fn str_macro<T>(_: T) -> &'static str {
    "alignas"
}

fn xstr_macro<T>(_: T) -> &'static str {
    "_Alignas"
}

fn main() {
    // Test alignment values
    assert_eq!(align_of_val(unsafe { &C }), MAX_ALIGN);
    assert_eq!(align_of_val(unsafe { &S }), MAX_ALIGN);
    assert_eq!(align_of_val(unsafe { &I }), 4);
    assert_eq!(align_of_val(unsafe { &L }), MAX_ALIGN);
    assert_eq!(align_of_val(unsafe { &LL }), MAX_ALIGN);
    assert_eq!(align_of_val(unsafe { &F }), MAX_ALIGN);
    assert_eq!(align_of_val(unsafe { &D }), MAX_ALIGN);
    assert_eq!(align_of_val(unsafe { &CLD }), MAX_ALIGN);
    assert_eq!(align_of_val(unsafe { &CA }), 4);
    assert_eq!(align_of_val(unsafe { &X }), MAX_ALIGN);
    assert_eq!(align_of_val(unsafe { &Y }), MAX_ALIGN);
    
    // Test macro expansions
    let s1 = xstr_macro(alignas);
    let s2 = "align_of";
    let s3 = "1";
    let s4 = "1";
    
    if s1 != "_Alignas" {
        std::process::exit(1);
    }
    if s2 != "_Alignof" {
        std::process::exit(1);
    }
    if s3 != "1" {
        std::process::exit(1);
    }
    if s4 != "1" {
        std::process::exit(1);
    }
    
    // Local variable with alignment
    #[repr(align(MAX_ALIGN))]
    struct AlignedLL(i64);
    let _auto_ll = AlignedLL(0);
    
    // Check alignments are defined
    assert!(MAX_ALIGN > 0);
    assert_eq!(align_of::<i32>(), 4);
}