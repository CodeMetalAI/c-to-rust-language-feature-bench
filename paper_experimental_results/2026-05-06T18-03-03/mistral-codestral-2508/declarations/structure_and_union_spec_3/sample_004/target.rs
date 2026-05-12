use std::mem;

struct S {
    i: i32,
    a: [i32; 0], // Zero-sized array to represent the flexible array member
}

fn main() {
    let mut p: Box<S> = unsafe {
        // Allocate memory for the struct and 4 additional i32 elements
        let layout = std::alloc::Layout::from_size_align(
            mem::size_of::<S>() + 4 * mem::size_of::<i32>(),
            mem::align_of::<S>(),
        ).unwrap();
        let ptr = std::alloc::alloc(layout);
        if ptr.is_null() {
            std::process::exit(3);
        }
        Box::from_raw(ptr as *mut S)
    };

    // Check the offsets
    if mem::offset_of!(S, i) != 0 {
        std::process::exit(1);
    }

    if mem::offset_of!(S, a) != mem::size_of::<S>() {
        std::process::exit(2);
    }

    // Initialize the values
    p.i = 7;
    unsafe {
        // Access the flexible array member
        let a_ptr = p.a.as_ptr() as *mut i32;
        *a_ptr = 11;
        *(a_ptr.offset(3)) = 22;
    }

    // Verify the values
    if p.i != 7 || unsafe { *p.a.as_ptr() } != 11 || unsafe { *p.a.as_ptr().offset(3) } != 22 {
        std::process::exit(4);
    }

    // Deallocate the memory
    let layout = std::alloc::Layout::from_size_align(
        mem::size_of::<S>() + 4 * mem::size_of::<i32>(),
        mem::align_of::<S>(),
    ).unwrap();
    unsafe {
        std::alloc::dealloc(Box::into_raw(p) as *mut u8, layout);
    }

    std::process::exit(0);
}