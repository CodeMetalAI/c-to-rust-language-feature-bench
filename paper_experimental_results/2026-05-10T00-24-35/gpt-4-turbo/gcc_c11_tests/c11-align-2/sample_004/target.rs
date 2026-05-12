use std::mem::align_of;

// In Rust, the `align_of` and `align_as` functionality is built directly into the language and its standard library.

#[repr(align(8))]
static mut C: char = '0';

#[repr(align(8))]
static mut S: i16 = 0;

#[repr(align(4))]
static mut I: i32 = 0;

#[repr(align(8))]
static mut L: i64 = 0;

#[repr(align(8))]
static mut LL: i64 = 0;

#[repr(align(8))]
static mut F: f32 = 0.0;

#[repr(align(8))]
static mut D: f64 = 0.0;

// The equivalent of _Complex long double is not straightforwardly available in safe Rust.
// However, for the purpose of alignment, this can be represented by a tuple of two f64 values.
#[repr(align(16))]
static mut CLD: (f64, f64) = (0.0, 0.0);

#[repr(align(4))] // The highest alignment specified (char, int)
static mut CA: [char; 10] = ['\0'; 10];

static mut X: i32 = 0; // Aligned as per max_align_t

const ALIGNOF_MAX_ALIGN_T: usize = 16;
const E: usize = ALIGNOF_MAX_ALIGN_T;

#[repr(align(16))]
static mut Y: i32 = 0;

fn main() {
    assert_eq!(align_of::<char>(), 1);
    assert_eq!(align_of::<i16>(), 2);
    assert_eq!(align_of::<i32>(), 4);
    assert_eq!(align_of::<i64>(), 8);
    assert_eq!(align_of::<f32>(), 4);
    assert_eq!(align_of::<f64>(), 8);
    assert_eq!(align_of::<(f64, f64)>(), 16);

    // Verify alignment of arrays and struct equivalents
    assert_eq!(align_of::<[char; 10]>(), 1);
    assert_eq!(align_of::<i32>(), 4);

    // Check alignment directly using variable instances
    unsafe {
        assert_eq!(align_of_val(&C), 8);
        assert_eq!(align_of_val(&S), 8);
        assert_eq!(align_of_val(&I), 4);
        assert_eq!(align_of_val(&L), 8);
        assert_eq!(align_of_val(&LL), 8);
        assert_eq!(align_of_val(&F), 8);
        assert_eq!(align_of_val(&D), 8);
        assert_eq!(align_of_val(&CLD), 16);
        assert_eq!(align_of_val(&CA), 4);
        assert_eq!(align_of_val(&X), 4);
        assert_eq!(align_of_val(&Y), 16);
    }
}

// Helper functions to get the alignment of a value by reference
fn align_of_val<T>(_: &T) -> usize {
    align_of::<T>()
}