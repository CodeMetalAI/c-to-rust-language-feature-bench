use std::alloc::Layout;
use std::mem::{align_of, size_of};

// Simulate max_align_t alignment - use the largest natural alignment
const MAX_ALIGN: usize = {
    let mut max = align_of::<u8>();
    if align_of::<u16>() > max { max = align_of::<u16>(); }
    if align_of::<u32>() > max { max = align_of::<u32>(); }
    if align_of::<u64>() > max { max = align_of::<u64>(); }
    if align_of::<usize>() > max { max = align_of::<usize>(); }
    if align_of::<f32>() > max { max = align_of::<f32>(); }
    if align_of::<f64>() > max { max = align_of::<f64>(); }
    max
};

#[repr(C, align(16))]
struct AlignedChar {
    _value: i8,
}

#[repr(C, align(16))]
struct AlignedShort {
    _value: i16,
}

#[repr(C, align(4))]
struct AlignedInt {
    _value: i32,
}

#[repr(C, align(16))]
struct AlignedLong {
    _value: i64,
}

#[repr(C, align(16))]
struct AlignedLongLong {
    _value: i64,
}

#[repr(C, align(16))]
struct AlignedFloat {
    _value: f32,
}

#[repr(C, align(16))]
struct AlignedDouble {
    _value: f64,
}

#[repr(C, align(16))]
struct AlignedComplexLongDouble {
    _real: f64,
    _imag: f64,
}

#[repr(C, align(4))]
struct AlignedCharArray {
    _value: [i8; 10],
}

#[repr(C, align(16))]
struct AlignedX {
    _value: i32,
}

#[repr(C, align(16))]
struct AlignedY {
    _value: i32,
}

static mut C: AlignedChar = AlignedChar { _value: 0 };
static mut S: AlignedShort = AlignedShort { _value: 0 };
static mut I: AlignedInt = AlignedInt { _value: 0 };
static mut L: AlignedLong = AlignedLong { _value: 0 };
static mut LL: AlignedLongLong = AlignedLongLong { _value: 0 };
static mut F: AlignedFloat = AlignedFloat { _value: 0.0 };
static mut D: AlignedDouble = AlignedDouble { _value: 0.0 };
static mut CLD: AlignedComplexLongDouble = AlignedComplexLongDouble { _real: 0.0, _imag: 0.0 };
static mut CA: AlignedCharArray = AlignedCharArray { _value: [0; 10] };
static mut X: AlignedX = AlignedX { _value: 0 };
static mut Y: AlignedY = AlignedY { _value: 0 };

fn func() {
    #[repr(C, align(16))]
    struct AlignedAutoLL {
        _value: i64,
    }
    let _auto_ll = AlignedAutoLL { _value: 0 };
}

const S1: &str = "_Alignas";
const S2: &str = "_Alignof";
const S3: &str = "1";
const S4: &str = "1";

fn main() {
    if S1 != "_Alignas" {
        std::process::abort();
    }
    if S2 != "_Alignof" {
        std::process::abort();
    }
    if S3 != "1" {
        std::process::abort();
    }
    if S4 != "1" {
        std::process::abort();
    }
}