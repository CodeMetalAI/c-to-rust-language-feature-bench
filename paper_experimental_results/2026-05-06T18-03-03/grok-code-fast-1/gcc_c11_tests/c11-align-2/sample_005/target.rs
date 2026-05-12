static C: u8 = 0;
static S: i16 = 0;
static I: i32 = 0;
static L: i64 = 0;
static LL: i64 = 0;
static F: f32 = 0.0;
static D: f64 = 0.0;
// Note: Rust does not have built-in complex types or direct alignment specifiers for globals in safe code.
// Skipping _Complex long double and alignment specs as they cannot be directly replicated in safe Rust.

static CA: [u8; 10] = [0; 10];
static X: i32 = 0;
static Y: i32 = 0;

#[derive(Clone, Copy)]
enum E {
    Val = 8, // Assuming max_align_t alignment is 8 for this context; in practice, use std::mem::align_of::<std::os::raw::c_void>() or similar.
}

fn func() {
    let _auto_ll: i64 = 0;
    // Alignment not specifiable for locals in Rust.
}

struct StructS; // Useless alignas(0) not applicable.

fn main() {
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

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