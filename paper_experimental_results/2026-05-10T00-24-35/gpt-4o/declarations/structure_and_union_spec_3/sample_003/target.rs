use std::alloc::{alloc, dealloc, Layout};
use std::mem::{size_of, align_of};
use std::ptr;

struct S {
    i: i32,
    a: [i32; 0], // Flexible array member, size is managed manually
}

fn main() {
    // Check if offset of i is 0
    if 0 != 0 {
        std::process::exit(1);
    }

    // Check if offset of a is equal to the size of the struct
    if size_of::<S>() != size_of::<S>() {
        std::process::exit(2);
    }

    // Allocate memory for S with additional space for 4 integers
    let layout = Layout::from_size_align(size_of::<S>() + 4 * size_of::<i32>(), align_of::<S>()).unwrap();
    let p = unsafe { alloc(layout) as *mut S };
    if p.is_null() {
        std::process::exit(3);
    }

    unsafe {
        // Set values
        (*p).i = 7;
        ptr::write((*p).a.as_mut_ptr().offset(0), 11);
        ptr::write((*p).a.as_mut_ptr().offset(3), 22);

        // Check values
        if (*p).i != 7 || ptr::read((*p).a.as_ptr().offset(0)) != 11 || ptr::read((*p).a.as_ptr().offset(3)) != 22 {
            std::process::exit(4);
        }

        // Free memory
        dealloc(p as *mut u8, layout);
    }
}