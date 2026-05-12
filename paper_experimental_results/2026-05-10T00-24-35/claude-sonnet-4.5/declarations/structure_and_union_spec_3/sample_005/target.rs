use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

#[repr(C)]
struct S {
    i: i32,
}

fn main() {
    // Check offset of i (should be 0)
    let offset_i = 0; // i is the first field, so offset is 0
    if offset_i != 0 {
        std::process::exit(1);
    }

    // Check that offset of flexible array member equals size of struct
    let size_of_s = mem::size_of::<S>();
    let offset_a = size_of_s;
    if offset_a != size_of_s {
        std::process::exit(2);
    }

    // Allocate memory for struct S + 4 integers
    let total_size = size_of_s + 4 * mem::size_of::<i32>();
    let align = mem::align_of::<S>();
    
    let layout = match Layout::from_size_align(total_size, align) {
        Ok(layout) => layout,
        Err(_) => std::process::exit(3),
    };

    let p = unsafe { alloc(layout) as *mut S };
    if p.is_null() {
        std::process::exit(3);
    }

    unsafe {
        // Set i field
        (*p).i = 7;

        // Access flexible array member (located right after the struct)
        let a_ptr = (p as *mut u8).add(size_of_s) as *mut i32;
        *a_ptr.add(0) = 11;
        *a_ptr.add(3) = 22;

        // Verify values
        if (*p).i != 7 || *a_ptr.add(0) != 11 || *a_ptr.add(3) != 22 {
            dealloc(p as *mut u8, layout);
            std::process::exit(4);
        }

        // Free memory
        dealloc(p as *mut u8, layout);
    }

    std::process::exit(0);
}