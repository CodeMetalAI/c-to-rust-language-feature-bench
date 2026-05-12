use std::mem;

const _: () = {
    let _c: [u8; mem::align_of::<usize>()] = [0u8; mem::align_of::<usize>()];
};

enum E { E = mem::align_of::<usize>() }

fn func() {
    let _ll: [u8; mem::align_of::<usize>()] = [0u8; mem::align_of::<usize>()];
}

#[repr(C, align(mem::align_of::<usize>()))]
struct S;

#[repr(C, align(0))]
struct U;

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