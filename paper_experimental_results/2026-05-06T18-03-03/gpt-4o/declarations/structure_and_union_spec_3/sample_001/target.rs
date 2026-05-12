use std::mem;
use std::ptr;

struct S {
    i: i32,
    a: [i32; 0],
}

fn main() {
    // Check the offset of `i` in the struct
    if mem::size_of::<i32>() != 0 {
        std::process::exit(1);
    }

    // Check the offset of `a` in the struct
    if mem::size_of::<S>() != mem::size_of::<i32>() {
        std::process::exit(2);
    }

    // Allocate memory for `S` with additional space for 4 integers
    let layout = std::alloc::Layout::array::<i32>(4).unwrap();
    let size = mem::size_of::<S>() + layout.size();
    let ptr = unsafe { std::alloc::alloc(layout) as *mut S };

    if ptr.is_null() {
        std::process::exit(3);
    }

    unsafe {
        // Initialize the struct
        ptr::write(&mut (*ptr).i, 7);
        let a_ptr = (*ptr).a.as_mut_ptr() as *mut i32;
        ptr::write(a_ptr.add(0), 11);
        ptr::write(a_ptr.add(3), 22);

        // Check the values
        if (*ptr).i != 7 || *a_ptr.add(0) != 11 || *a_ptr.add(3) != 22 {
            std::alloc::dealloc(ptr as *mut u8, layout);
            std::process::exit(4);
        }

        // Deallocate memory
        std::alloc::dealloc(ptr as *mut u8, layout);
    }

    std::process::exit(0);
}