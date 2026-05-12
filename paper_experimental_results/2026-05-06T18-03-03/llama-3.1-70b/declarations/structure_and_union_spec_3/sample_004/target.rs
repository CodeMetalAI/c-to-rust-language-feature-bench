fn main() {
    #[repr(C)]
    struct S {
        inner: Inner,
        a: [i32; 0],
    }

    #[repr(C)]
    struct Inner {
        i: i32,
    }

    let p = std::alloc::alloc(std::mem::size_of::<S>() + 4 * std::mem::size_of::<i32>()) as *mut S;
    if p.is_null() {
        std::process::exit(3);
    }

    unsafe {
        (*p).inner.i = 7;
        (*p).a.as_mut_ptr().offset(0).write(11);
        (*p).a.as_mut_ptr().offset(3).write(22);

        if (*p).inner.i!= 7 || (*p).a.as_ptr().offset(0).read()!= 11 || (*p).a.as_ptr().offset(3).read()!= 22 {
            std::process::exit(4);
        }
    }

    std::alloc::dealloc(p as *mut u8, std::alloc::Layout::from_size_align(std::mem::size_of::<S>() + 4 * std::mem::size_of::<i32>(), std::mem::align_of::<S>()).unwrap());
    std::process::exit(0);
}