#![feature(align_offset)]
#![feature(abi_x86_interrupt)]

extern "C" {
    fn strcmp(a: *const u8, b: *const u8) -> i32;
    fn exit(code: i32);
    fn abort();
}

#[repr(C, align(max))]
struct MaxAlign;

static mut C: [u8; std::mem::align_of::<MaxAlign>()] = [0; std::mem::align_of::<MaxAlign>()];

#[repr(C, align(max))]
static mut S: [u8; std::mem::align_of::<MaxAlign>()] = [0; std::mem::align_of::<MaxAlign>()];

static mut I: i32 = 0;

static mut L: i64 = 0;

static mut LL: i64 = 0;

static mut F: f32 = 0.0;

static mut D: f64 = 0.0;

static mut CLD: f64 = 0.0;

static mut CA: [u8; 10] = [0; 10];

static mut X: i32 = 0;

enum E {
    E = std::mem::align_of::<MaxAlign>(),
}

#[repr(C, align(E as usize))]
static mut Y: i32 = 0;

fn func() {
    let _auto_ll: [u8; std::mem::align_of::<MaxAlign>()] = [0; std::mem::align_of::<MaxAlign>()];
}

#[repr(C, align(0))]
struct S;

fn main() {
    if strcmp("_Alignas\0".as_ptr(), "_Alignas\0".as_ptr())!= 0 {
        unsafe { abort(); }
    }
    if strcmp("_Alignof\0".as_ptr(), "_Alignof\0".as_ptr())!= 0 {
        unsafe { abort(); }
    }
    if strcmp("1\0".as_ptr(), "1\0".as_ptr())!= 0 {
        unsafe { abort(); }
    }
    if strcmp("1\0".as_ptr(), "1\0".as_ptr())!= 0 {
        unsafe { abort(); }
    }
}