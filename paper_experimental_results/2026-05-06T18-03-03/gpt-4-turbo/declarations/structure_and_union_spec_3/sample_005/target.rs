use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::mem;
use std::ptr;

struct S {
    i: i32,
    a: [i32; 0], // Zero-sized array to mimic flexible array member
}

fn main() {
    // Check offsets
    assert_eq!(mem::size_of::<i32>(), 4); // Ensure the size of i32 is 4 bytes
    assert_eq!(mem::align_of::<i32>(), 4); // Ensure alignment of i32 is 4 bytes

    // Allocate memory for S with additional space for 4 i32s
    let layout = Layout::from_size_align(mem::size_of::<S>() + 4 * mem::size_of::<i32>(), mem::align_of::<S>()).unwrap();
    let ptr = unsafe { alloc_zeroed(layout) } as *mut S;
    if ptr.is_null() {
        std::process::exit(3);
    }

    // Initialize values
    unsafe {
        (*ptr).i = 7;
        ptr.add(1).cast::<i32>().write(11); // Equivalent to p->a[0] = 11;
        ptr.add(1).cast::<i32>().add(3).write(22); // Equivalent to p->a[3] = 22;

        // Check values
        if (*ptr).i != 7 || ptr.add(1).cast::<i32>().read() != 11 || ptr.add(1).cast::<i32>().add(3).read() != 22 {
            std::process::exit(4);
        }
    }

    // Free memory
    unsafe {
        dealloc(ptr.cast::<u8>(), layout);
    }
}