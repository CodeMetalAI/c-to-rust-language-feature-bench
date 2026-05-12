use std::cmp::Ordering;

#[repr(align(16))] // Assuming max_align_t is 16, as often the case
static mut C: u8 = 0; // char c;
#[repr(align(16))]
static mut S: u16 = 0; // short s;
static mut I: i32 = 0; // int i;
#[repr(align(16))]
static mut L: i64 = 0; // long l;
#[repr(align(16))]
static mut LL: i64 = 0; // long long ll;
#[repr(align(4))]
static mut F: f32 = 0.0; // float f;
#[repr(align(8))]
static mut D: f64 = 0.0; // double d;
#[repr(align(16))]
static mut CLD: f64 = 0.0; // _Complex long double cld;
static mut CA: [u8; 10] = [0; 10]; // char ca[10];
#[repr(align(4))]
static mut X: i32 = 0;  // int x;
#[repr(align(16))]
static mut Y: i32 = 0; // int y;

fn main() {
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    if compare_strings(s1, "_Alignas") != Ordering::Equal {
        std::process::abort();
    }
    if compare_strings(s2, "_Alignof") != Ordering::Equal {
        std::process::abort();
    }
    if compare_strings(s3, "1") != Ordering::Equal {
        std::process::abort();
    }
    if compare_strings(s4, "1") != Ordering::Equal {
        std::process::abort();
    }
}

fn compare_strings(s1: &str, s2: &str) -> Ordering {
    s1.cmp(s2)
}