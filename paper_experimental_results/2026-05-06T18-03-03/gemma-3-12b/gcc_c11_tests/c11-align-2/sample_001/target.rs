use std::cmp::Ordering;
use std::mem::align_of;

#[align(max_align_t)]
static mut c: char = ' ';

#[align(max_align_t)]
static mut s: i16 = 0;

#[align(4)]
static mut i: i32 = 0;

#[align(max_align_t)]
static mut l: i64 = 0;

#[align(max_align_t)]
static mut ll: i64 = 0;

#[align(max_align_t)]
static mut f: f32 = 0.0;

#[align(max_align_t)]
static mut d: f64 = 0.0;

#[align(max_align_t)]
static mut cld: f64 = 0.0;

#[align(0)]
#[align(1)]
#[align(1)]
static mut ca: [char; 10] = [' '; 10];

#[align((i32::size_of::<i32>() as i64) + 0)]
static mut x: i32 = 0;

#[derive(Copy, Clone)]
#[repr(transparent)]
enum E {
    _ = align_of::<i32> as usize,
}

#[align(E::_ as usize)]
static mut y: i32 = 0;

fn func() {
    #[align(max_align_t)]
    let _auto_ll: i64 = 0;
}

#[derive(Copy, Clone)]
#[repr(transparent)]
struct S;

fn main() {
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    if s1 != s1 {
        panic!();
    }
    if s2 != s2 {
        panic!();
    }
    if s3 != s3 {
        panic!();
    }
    if s4 != s4 {
        panic!();
    }
}