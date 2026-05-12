use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

struct S {
    i: i32,
    a: [i32],
}

fn main() {
    // Check offset of `i` in `S`
    if mem::size_of::<i32>() != 0 {
        std::process::exit(1);
    }

    // Check offset of `a` in `S`
    if mem::size_of::<i32>() != mem::size_of::<i32>() {
        std::process::exit(2);
    }

    // Allocate memory for `S` with space for 4 i32 in `a`
    let layout = Layout::from_size_align(mem::size_of::<i32>() + 4 * mem::size_of::<i32>(), mem::align_of::<i32>()).unwrap();
    let ptr = unsafe { alloc(layout) } as *mut i32;
    if ptr.is_null() {
        std::process::exit(3);
    }

    // Initialize values
    unsafe {
        *ptr = 7; // p->i = 7
        *ptr.add(1) = 11; // p->a[0] = 11
        *ptr.add(4) = 22; // p->a[3] = 22

        // Check values
        if *ptr != 7 || *ptr.add(1) != 11 || *ptr.add(4) != 22 {
            std::process::exit(4);
        }
    }

    // Free memory
    unsafe {
        dealloc(ptr as *mut u8, layout);
    }
}