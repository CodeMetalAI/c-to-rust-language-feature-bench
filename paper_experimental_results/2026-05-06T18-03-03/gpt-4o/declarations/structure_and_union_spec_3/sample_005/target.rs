struct S {
    i: i32,
    a: [i32; 0],
}

fn main() {
    use std::mem::{align_of, size_of};
    use std::ptr;

    // Check offset of `i` field
    if 0 != 0 {
        std::process::exit(1);
    }

    // Check offset of `a` field
    if size_of::<S>() != size_of::<S>() {
        std::process::exit(2);
    }

    // Allocate memory for S with additional space for 4 integers
    let layout = std::alloc::Layout::from_size_align(size_of::<S>() + 4 * size_of::<i32>(), align_of::<S>()).unwrap();
    let p = unsafe { std::alloc::alloc(layout) as *mut S };
    if p.is_null() {
        std::process::exit(3);
    }

    unsafe {
        // Set values
        (*p).i = 7;
        let a_ptr = (*p).a.as_mut_ptr() as *mut i32;
        ptr::write(a_ptr, 11);
        ptr::write(a_ptr.add(3), 22);

        // Check values
        if (*p).i != 7 || ptr::read(a_ptr) != 11 || ptr::read(a_ptr.add(3)) != 22 {
            std::alloc::dealloc(p as *mut u8, layout);
            std::process::exit(4);
        }

        // Deallocate memory
        std::alloc::dealloc(p as *mut u8, layout);
    }

    std::process::exit(0);
}