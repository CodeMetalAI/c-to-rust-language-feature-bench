fn main() {
    #[repr(C)]
    struct Inner {
        i: i32,
    }

    #[repr(C)]
    struct S {
        inner: Inner,
        a: [i32; 0],
    }

    // Check offset of `i`
    if std::mem::size_of::<Inner>() != 0 {
        return std::process::exit(1);
    }

    // Check offset of `a`
    if std::mem::size_of::<S>() != std::mem::size_of::<Inner>() {
        return std::process::exit(2);
    }

    // Allocate memory for `S` with additional space for 4 `i32`
    let layout = std::alloc::Layout::from_size_align(
        std::mem::size_of::<S>() + 4 * std::mem::size_of::<i32>(),
        std::mem::align_of::<S>(),
    ).unwrap();
    let p = unsafe { std::alloc::alloc(layout) } as *mut S;
    if p.is_null() {
        return std::process::exit(3);
    }

    // Safe access through raw pointers
    unsafe {
        (*p).inner.i = 7;
        (*p).a.as_mut_ptr().offset(0).write(11);
        (*p).a.as_mut_ptr().offset(3).write(22);

        if (*p).inner.i != 7 || (*p).a.as_ptr().offset(0).read() != 11 || (*p).a.as_ptr().offset(3).read() != 22 {
            return std::process::exit(4);
        }
    }

    // Free allocated memory
    unsafe {
        std::alloc::dealloc(p as *mut u8, layout);
    }
}