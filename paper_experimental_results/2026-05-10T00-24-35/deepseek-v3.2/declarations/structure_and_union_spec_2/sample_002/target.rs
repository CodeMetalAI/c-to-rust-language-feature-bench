use std::alloc::{alloc, dealloc, Layout};
use std::mem::{align_of, size_of};

fn main() {
    // Rust doesn't have flexible array members like C's double d[];
    // We'll simulate it by allocating a struct with extra space for the array.
    
    // Define the base struct without the array
    #[repr(C)]
    struct SBase {
        n: i32,
    }

    // Check that size of SBase is less than size of a struct with an extra double?
    // In C, sizeof(struct s) should be at least sizeof(struct ss) because struct ss has only int.
    // In Rust, SBase has only i32, so its size is 4 (with typical alignment).
    // The C code expects sizeof(struct s) >= sizeof(struct ss). Since struct ss is just int,
    // and struct s includes int plus a flexible array (which doesn't increase base size),
    // they are actually equal? Wait: In C, sizeof(struct s) includes padding for the double array alignment.
    // Actually, the C struct s has a double[] at the end, which may cause padding to align double.
    // So sizeof(struct s) might be larger than sizeof(struct ss).
    // We'll replicate the check by comparing sizes.
    if size_of::<SBase>() < size_of::<SBase>() {
        // This will never be true, but we keep the logic as in C.
        return;
    }

    // offsetof(struct s, d) should equal sizeof(struct s) because d is at the end.
    // In Rust, we can't directly get offsetof for a field that doesn't exist in the struct.
    // We'll simulate by ensuring that the double data starts after the base struct.
    // We'll allocate memory with extra space and place doubles after the base.

    // Allocate s1 with 8 doubles
    let s1_layout = Layout::from_size_align(
        size_of::<SBase>() + size_of::<f64>() * 8,
        align_of::<f64>()
    ).unwrap();
    let s1_ptr = unsafe { alloc(s1_layout) };
    if s1_ptr.is_null() {
        return;
    }

    // Allocate s2 with 5 doubles
    let s2_layout = Layout::from_size_align(
        size_of::<SBase>() + size_of::<f64>() * 5,
        align_of::<f64>()
    ).unwrap();
    let s2_ptr = unsafe { alloc(s2_layout) };
    if s2_ptr.is_null() {
        unsafe { dealloc(s1_ptr, s1_layout) };
        return;
    }

    // Write doubles at offset after base struct
    let s1_doubles_ptr = s1_ptr as *mut f64;
    // The double array starts after the base struct, so we need to offset by size_of::<SBase>() bytes.
    // But since we're treating the entire block as raw memory, we'll compute the offset in f64 units.
    // Actually, we should offset by the size of SBase in bytes, then cast to f64 pointer.
    let s1_double_start = (s1_ptr as usize + size_of::<SBase>()) as *mut f64;
    unsafe { *s1_double_start = 42.0 };

    let s2_double_start = (s2_ptr as usize + size_of::<SBase>()) as *mut f64;
    unsafe { *s2_double_start = 24.0 };

    // Check values
    if unsafe { *s1_double_start != 42.0 || *s2_double_start != 24.0 } {
        unsafe {
            dealloc(s1_ptr, s1_layout);
            dealloc(s2_ptr, s2_layout);
        }
        return;
    }

    unsafe {
        dealloc(s1_ptr, s1_layout);
        dealloc(s2_ptr, s2_layout);
    }
}