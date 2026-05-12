use std::ffi::CStr;
use std::os::raw::c_char;

const fn align_of_max_align_t() -> usize {
    std::mem::align_of::<usize>()
}

const S1: &CStr = CStr::from_bytes_with_nul(b"_Alignas\0").unwrap();
const S2: &CStr = CStr::from_bytes_with_nul(b"_Alignof\0").unwrap();
const S3: &CStr = CStr::from_bytes_with_nul(b"1\0").unwrap();
const S4: &CStr = CStr::from_bytes_with_nul(b"1\0").unwrap();

#[repr(align(align_of_max_align_t()))]
struct MaxAlignStruct;

#[repr(align(align_of_max_align_t()))]
struct MaxAlignShort;

#[repr(align(align_of_max_align_t()))]
struct MaxAlignLong;

#[repr(align(align_of_max_align_t()))]
struct MaxAlignLongLong;

#[repr(align(align_of_max_align_t()))]
struct MaxAlignFloat;

#[repr(align(align_of_max_align_t()))]
struct MaxAlignDouble;

#[repr(align(align_of_max_align_t()))]
struct MaxAlignComplexLongDouble;

#[repr(align(0))]
struct UselessStruct;

fn main() {
    let s1 = S1.to_str().unwrap();
    let s2 = S2.to_str().unwrap();
    let s3 = S3.to_str().unwrap();
    let s4 = S4.to_str().unwrap();

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