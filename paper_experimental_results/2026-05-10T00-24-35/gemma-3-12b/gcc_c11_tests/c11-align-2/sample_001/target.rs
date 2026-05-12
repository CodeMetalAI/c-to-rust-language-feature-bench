#[repr(align(max_align_t))]
static mut c: char = 0;

#[repr(align(max_align_t))]
static mut s: i16 = 0;

#[repr(align(4))]
static mut i: i32 = 0;

#[repr(align(max_align_t))]
static mut l: i64 = 0;

#[repr(align(max_align_t))]
static mut ll: i64 = 0;

#[repr(align(max_align_t))]
static mut f: f32 = 0.0;

#[repr(align(max_align_t))]
static mut d: f64 = 0.0;

#[repr(align(max_align_t))]
static mut cld: std::complex::Complex<f64> = std::complex::Complex { re: 0.0, im: 0.0 };

#[repr(align(0))]
#[repr(align(1))]
#[repr(align(4))]
static mut ca: [char; 10] = [0; 10];

#[repr(align(std::mem::align_of::<max_align_t>() + 0)]
static mut x: i32 = 0;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum e {
    E(usize),
}

#[repr(align(e::E as usize))]
static mut y: i32 = 0;

fn func() {
    #[repr(align(max_align_t))]
    let auto_ll: i64 = 0;
}

#[repr(align(0))]
struct s;

fn main() {
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    if s1 != s1 {
        std::process::abort();
    }
    if s2 != s2 {
        std::process::abort();
    }
    if s3 != s3 {
        std::process::abort();
    }
    if s4 != s4 {
        std::process::abort();
    }
}