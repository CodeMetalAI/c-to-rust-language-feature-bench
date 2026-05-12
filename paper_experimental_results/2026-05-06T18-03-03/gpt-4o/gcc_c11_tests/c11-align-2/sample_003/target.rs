use std::mem;

fn main() {
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    if s1 != "_Alignas" {
        std::process::abort();
    }
    if s2 != "_Alignof" {
        std::process::abort();
    }
    if s3 != "1" {
        std::process::abort();
    }
    if s4 != "1" {
        std::process::abort();
    }
}

#[repr(align(align_of_max_align_t()))]
struct AlignedChar {
    _c: u8,
}

#[repr(align(align_of_max_align_t()))]
struct AlignedShort {
    _s: i16,
}

#[repr(align(align_of_max_align_t()))]
struct AlignedLong {
    _l: i64,
}

#[repr(align(align_of_max_align_t()))]
struct AlignedLongLong {
    _ll: i64,
}

#[repr(align(align_of_max_align_t()))]
struct AlignedFloat {
    _f: f32,
}

#[repr(align(align_of_max_align_t()))]
struct AlignedDouble {
    _d: f64,
}

#[repr(align(align_of_max_align_t()))]
struct AlignedComplexLongDouble {
    _cld: (f64, f64),
}

#[repr(align(align_of_max_align_t()))]
struct AlignedArray {
    _ca: [u8; 10],
}

#[repr(align(align_of_max_align_t()))]
struct AlignedInt {
    _x: i32,
}

#[repr(align(align_of_max_align_t()))]
struct AlignedEnumInt {
    _y: i32,
}

fn align_of_max_align_t() -> usize {
    mem::align_of::<f64>()
}