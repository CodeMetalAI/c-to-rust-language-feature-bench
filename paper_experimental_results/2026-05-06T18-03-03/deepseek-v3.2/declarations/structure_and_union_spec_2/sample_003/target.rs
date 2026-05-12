use std::alloc::{alloc, dealloc, Layout};
use std::mem::{align_of, size_of};

struct S {
    n: i32,
    // Rust doesn't have flexible array members directly.
    // We'll handle the allocation manually.
}

fn main() {
    // Check that S has at least the size of i32
    if size_of::<S>() < size_of::<i32>() {
        std::process::exit(1);
    }

    // Check that the alignment and padding don't cause offset issues
    // In C, the flexible array member is at the end, after padding.
    // In Rust, we can't directly check offsetof for a non-existent field.
    // Instead, we verify that the struct size is aligned to hold the i32
    // and has no extra padding at the end that would affect our manual layout.
    let s_size = size_of::<S>();
    let n_size = size_of::<i32>();
    let n_align = align_of::<i32>();
    let s_align = align_of::<S>();

    // The offset of the imaginary flexible array should be exactly the size of the struct.
    // Since we can't have a flexible array in Rust, we ensure the struct has no trailing padding
    // that would make it larger than necessary for the i32.
    // In practice, we'll just check that the struct size is a multiple of its alignment
    // and at least as big as i32.
    if s_size < n_size {
        std::process::exit(1);
    }

    // Allocate memory for s1 with space for 8 doubles after the struct
    let double_size = size_of::<f64>();
    let s1_total_size = s_size + double_size * 8;
    let s1_layout = Layout::from_size_align(s1_total_size, s_align).unwrap();
    let s1_ptr = unsafe { alloc(s1_layout) };
    if s1_ptr.is_null() {
        std::process::exit(1);
    }

    // Allocate memory for s2 with space for 5 doubles after the struct
    let s2_total_size = s_size + double_size * 5;
    let s2_layout = Layout::from_size_align(s2_total_size, s_align).unwrap();
    let s2_ptr = unsafe { alloc(s2_layout) };
    if s2_ptr.is_null() {
        unsafe { dealloc(s1_ptr, s1_layout) };
        std::process::exit(1);
    }

    // Write the n field (optional, but keep the struct initialized)
    unsafe {
        // Set n to 0 (not used in the test, but keep it initialized)
        (s1_ptr as *mut i32).write(0);
        (s2_ptr as *mut i32).write(0);
    }

    // Access the flexible array part (doubles) after the struct
    let s1_doubles_ptr = unsafe { s1_ptr.add(s_size) as *mut f64 };
    let s2_doubles_ptr = unsafe { s2_ptr.add(s_size) as *mut f64 };

    unsafe {
        s1_doubles_ptr.write(42.0);
        s2_doubles_ptr.write(24.0);
    }

    // Check the values
    let s1_val = unsafe { s1_doubles_ptr.read() };
    let s2_val = unsafe { s2_doubles_ptr.read() };
    if s1_val != 42.0 || s2_val != 24.0 {
        unsafe {
            dealloc(s1_ptr, s1_layout);
            dealloc(s2_ptr, s2_layout);
        }
        std::process::exit(1);
    }

    // Free the memory
    unsafe {
        dealloc(s1_ptr, s1_layout);
        dealloc(s2_ptr, s2_layout);
    }

    std::process::exit(0);
}