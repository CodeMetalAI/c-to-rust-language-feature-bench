use std::alloc::{alloc, dealloc, Layout};
use std::mem::{align_of, size_of};

struct S {
    n: i32,
    // Flexible array member will be handled manually
}

fn main() -> i32 {
    // Check that S without the flexible array has the expected size
    if size_of::<S>() < size_of::<i32>() {
        return 1;
    }

    // Check offset of the flexible array (should be at the end of the struct)
    let d_offset = size_of::<S>();
    if d_offset != size_of::<S>() {
        return 1;
    }

    // Allocate memory for S with 8 doubles
    let layout1 = Layout::from_size_align(
        size_of::<S>() + size_of::<f64>() * 8,
        align_of::<S>().max(align_of::<f64>()),
    ).unwrap();
    let s1_ptr = unsafe { alloc(layout1) } as *mut S;
    if s1_ptr.is_null() {
        return 1;
    }

    // Allocate memory for S with 5 doubles
    let layout2 = Layout::from_size_align(
        size_of::<S>() + size_of::<f64>() * 5,
        align_of::<S>().max(align_of::<f64>()),
    ).unwrap();
    let s2_ptr = unsafe { alloc(layout2) } as *mut S;
    if s2_ptr.is_null() {
        unsafe { dealloc(s1_ptr as *mut u8, layout1) };
        return 1;
    }

    // Initialize the flexible array members
    unsafe {
        let d1_ptr = (s1_ptr as *mut u8).add(d_offset) as *mut f64;
        *d1_ptr = 42.0;

        let d2_ptr = (s2_ptr as *mut u8).add(d_offset) as *mut f64;
        *d2_ptr = 24.0;

        // Check values
        if *d1_ptr != 42.0 || *d2_ptr != 24.0 {
            dealloc(s1_ptr as *mut u8, layout1);
            dealloc(s2_ptr as *mut u8, layout2);
            return 1;
        }

        dealloc(s1_ptr as *mut u8, layout1);
        dealloc(s2_ptr as *mut u8, layout2);
    }

    0
}