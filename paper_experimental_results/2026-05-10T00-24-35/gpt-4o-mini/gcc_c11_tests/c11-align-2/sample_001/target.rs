use std::cmp::Ordering;
use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(align(16))]
struct MaxAlign {
    c: u8,
}

#[repr(align(16))]
struct AlignedShort {
    s: i16,
}

#[repr(align(4))]
struct AlignedInt {
    i: i32,
}

#[repr(align(16))]
struct AlignedLong {
    l: i64,
}

#[repr(align(16))]
struct AlignedLongLong {
    ll: i128,
}

#[repr(align(16))]
struct AlignedFloat {
    f: f32,
}

#[repr(align(16))]
struct AlignedDouble {
    d: f64,
}

#[repr(align(16))]
struct AlignedComplexLongDouble {
    cld: f64, // Rust does not have complex numbers in the same way, using f64 as a placeholder
}

#[repr(align(0))]
struct Useless;

const fn alignas(val: &'static str) -> &'static str {
    val
}

const S1: &str = alignas("_Alignas");
const S2: &str = alignas("_Alignof");
const S3: &str = alignas("1");
const S4: &str = alignas("1");

fn main() {
    fn strcmp(s1: &str, s2: &str) -> Ordering {
        s1.cmp(s2)
    }

    if strcmp(S1, "_Alignas") != Ordering::Equal {
        std::process::abort();
    }
    if strcmp(S2, "_Alignof") != Ordering::Equal {
        std::process::abort();
    }
    if strcmp(S3, "1") != Ordering::Equal {
        std::process::abort();
    }
    if strcmp(S4, "1") != Ordering::Equal {
        std::process::abort();
    }
}