use std::cmp::Ordering;

#[repr(align(16))] // Assuming max_align_t is 16, as it often is.  This is a reasonable default.
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

const s1: &str = "_Alignas";
const s2: &str = "_Alignof";
const s3: &str = "1";
const s4: &str = "1";

fn main() {
    if s1 != "_Alignas" {
        panic!();
    }
    if s2 != "_Alignof" {
        panic!();
    }
    if s3 != "1" {
        panic!();
    }
    if s4 != "1" {
        panic!();
    }
}