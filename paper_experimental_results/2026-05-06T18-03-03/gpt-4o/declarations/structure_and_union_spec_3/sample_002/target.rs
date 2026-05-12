struct S {
    i: i32,
    a: [i32; 0],
}

fn main() {
    use std::mem;
    use std::ptr;

    // Check offset of `i`
    if mem::size_of::<i32>() != 0 {
        std::process::exit(1);
    }

    // Check offset of `a`
    if mem::size_of::<S>() != mem::size_of::<i32>() {
        std::process::exit(2);
    }

    // Allocate memory for `S` with additional space for 4 integers
    let size = mem::size_of::<S>() + 4 * mem::size_of::<i32>();
    let p = unsafe { libc::malloc(size) as *mut S };
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
            libc::free(p as *mut libc::c_void);
            std::process::exit(4);
        }

        // Free allocated memory
        libc::free(p as *mut libc::c_void);
    }

    std::process::exit(0);
}