use std::alloc::{alloc, dealloc, Layout};
use std::mem::{self, MaybeUninit};

struct S {
    inner: Inner,
    a: [i32; 0],
}

struct Inner {
    i: i32,
}

fn main() -> ! {
    // Check offsets
    assert_eq!(mem::offset_of!(S, inner.i), 0);
    assert_eq!(mem::offset_of!(S, a), mem::size_of::<S>());

    // Allocate memory for struct S + space for 4 integers
    let layout = Layout::from_size_align(mem::size_of::<S>() + 4 * mem::size_of::<i32>(), mem::align_of::<S>()).unwrap();
    let ptr = unsafe { alloc(layout) as *mut S };

    // Check allocation
    if ptr.is_null() {
        std::process::exit(3);
    }

    // Initialize inner values
    unsafe {
        ptr.write(MaybeUninit::zeroed().assume_init());
        (*ptr).inner.i = 7;
        (*ptr).a = [0; 0]; // This is just a placeholder, we will access the raw pointer for the allocated space

        let a_ptr = ((*ptr) as *mut i32).add(1); // Starting from the second element to accommodate the flexible array member
        *a_ptr = 11;
        *a_ptr.add(3) = 22;

        // Check values
        if (*ptr).inner.i != 7 || *a_ptr != 11 || *a_ptr.add(3) != 22 {
            std::process::exit(4);
        }
    }

    // Deallocate memory
    unsafe {
        dealloc(ptr as *mut u8, layout);
    }

    std::process::exit(0);
}