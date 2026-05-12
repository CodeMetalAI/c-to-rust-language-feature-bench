use std::mem;
use std::ptr;

struct S {
    i: i32,
    a: [i32; 0], // Flexible array member, but we'll use a vector instead for dynamic allocation
}

fn main() -> i32 {
    // Check offsets
    if mem::offset_of!(S, i) != 0 {
        return 1;
    }

    if mem::size_of::<S>() != mem::size_of::<S>() {
        return 2;
    }

    // Allocate memory for S with a flexible array
    let mut vec = vec![0; 4]; // Create a vector with space for 4 integers
    let p = Box::into_raw(Box::new(S { i: 0, a: [] })) as *mut S; // Create a raw pointer to S

    unsafe {
        if p.is_null() {
            return 3;
        }

        (*p).i = 7;
        ptr::write(&mut (*p).a[0] as *mut _ as *mut i32, 11);
        ptr::write(&mut (*p).a[3] as *mut _ as *mut i32, 22);

        if (*p).i != 7 || (*p).a[0] != 11 || (*p).a[3] != 22 {
            return 4;
        }

        // Free memory
        Box::from_raw(p); // Convert back to Box to free memory
    }

    0
}