use std::mem;

fn main() {
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

#[repr(align(8))]
struct MaxAlign;

#[repr(align(8))]
struct C;

#[repr(align(8))]
struct S;

#[repr(align(4))]
struct I;

#[repr(align(8))]
struct L;

#[repr(align(8))]
struct LL;

#[repr(align(8))]
struct F;

#[repr(align(8))]
struct D;

#[repr(align(8))]
struct CLD;

#[repr(align(1))]
struct CA([u8; 10]);

#[repr(align(8))]
struct X;

#[repr(align(8))]
struct Y;

fn func() {
    #[repr(align(8))]
    struct AutoLL;
}