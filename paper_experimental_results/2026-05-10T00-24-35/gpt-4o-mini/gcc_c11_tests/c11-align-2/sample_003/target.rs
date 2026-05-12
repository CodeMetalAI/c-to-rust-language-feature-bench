use std::mem;
use std::ptr;

const S1: &str = std::stringify!(alignas);
const S2: &str = std::stringify!(alignof);
const S3: &str = std::stringify!(__alignas_is_defined);
const S4: &str = std::stringify!(__alignof_is_defined);

fn main() {
    let c: [u8; mem::align_of::<usize>()] = [0; mem::align_of::<usize>()];
    let s: [i16; mem::align_of::<usize>()] = [0; mem::align_of::<usize>()];
    let i: i32 = 0;
    let l: i64 = 0;
    let ll: i128 = 0;
    let f: f32 = 0.0;
    let d: f64 = 0.0;
    let ca: [u8; 10] = [0; 10];
    let x: i32 = 0;
    let y: i32 = 0;

    // Check the alignment
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