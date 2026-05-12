use std::mem;

#[repr(align(16))]
struct AlignedData {
    c: u8,
    s: i16,
    i: i32,
    l: i64,
    ll: i64,
    f: f32,
    d: f64,
    cld: (f64, f64),
    ca: [u8; 10],
    x: i32,
    y: i32,
}

fn func() {
    let _auto_ll: [u8; mem::align_of::<i64>()] = [0; mem::align_of::<i64>()];
}

fn main() {
    let _aligned_data = AlignedData {
        c: 0,
        s: 0,
        i: 0,
        l: 0,
        ll: 0,
        f: 0.0,
        d: 0.0,
        cld: (0.0, 0.0),
        ca: [0; 10],
        x: 0,
        y: 0,
    };

    let s1 = stringify!(alignas);
    let s2 = stringify!(alignof);
    let s3 = stringify!(__alignas_is_defined);
    let s4 = stringify!(__alignof_is_defined);

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