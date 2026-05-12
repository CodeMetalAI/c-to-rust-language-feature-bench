use std::cmp::Ordering;

#[repr(align(16))] // Assuming max_align_t is 16, as it often is.  This is a reasonable guess.
#[allow(dead_code)]
static c: char = ' ';

#[repr(align(16))]
#[allow(dead_code)]
static s: i16 = 0;

#[repr(align(4))]
#[allow(dead_code)]
static i: i32 = 0;

#[repr(align(16))]
#[allow(dead_code)]
static l: i64 = 0;

#[repr(align(16))]
#[allow(dead_code)]
static ll: i64 = 0;

#[repr(align(4))]
#[allow(dead_code)]
static f: f32 = 0.0;

#[repr(align(8))]
#[allow(dead_code)]
static d: f64 = 0.0;

#[repr(align(16))]
#[allow(dead_code)]
static cld: f64 = 0.0;

#[repr(align(1))]
#[allow(dead_code)]
static ca: [char; 10] = [' '; 10];

#[repr(align(16))]
#[allow(dead_code)]
static x: i32 = 0;

#[derive(Copy, Clone)]
#[allow(dead_code)]
enum e { E, }

#[repr(align(16))]
#[allow(dead_code)]
static y: i32 = 0;

#[allow(dead_code)]
fn func() {
    #[repr(align(16))]
    let auto_ll: i64 = 0;
}

#[allow(dead_code)]
struct s;

fn main() {
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    if strcmp(s1, s2) != 0 {
        std::process::abort();
    }
    if strcmp(s3, s4) != 0 {
        std::process::abort();
    }
}

extern "C" {
    fn strcmp(s1: *const i8, s2: *const i8) -> i32;
    fn abort() ;
}