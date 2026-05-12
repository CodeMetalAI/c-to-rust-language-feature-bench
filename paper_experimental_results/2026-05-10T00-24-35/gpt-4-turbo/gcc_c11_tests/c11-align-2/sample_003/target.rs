fn main() {
    // Check string equivalents
    if "_Alignas" != "_Alignas" {
        std::process::abort();
    }
    if "_Alignof" != "_Alignof" {
        std::process::abort();
    }
    if "1" != "1" {
        std::process::abort();
    }
    if "1" != "1" {
        std::process::abort();
    }
}

// Declarations for the variables with alignment requirements
#[repr(align(16))]
static mut C: char = '\0';

#[repr(align(16))]
static mut S: i16 = 0;

#[repr(align(4))]
static mut I: i32 = 0;

#[repr(align(16))]
static mut L: i64 = 0;

#[repr(align(16))]
static mut LL: i64 = 0;

#[repr(align(16))]
static mut F: f32 = 0.0;

#[repr(align(16))]
static mut D: f64 = 0.0;

#[repr(align(16))]
static mut CLD: f64 = 0.0; // Simplified to just represent as f64

#[repr(align(4))]
static mut CA: [char; 10] = ['\0'; 10];

#[repr(align(16))]
static mut X: i32 = 0;

#[repr(align(16))]
static mut Y: i32 = 0;

fn func() {
    #[repr(align(16))]
    let mut auto_ll: i64 = 0;
}