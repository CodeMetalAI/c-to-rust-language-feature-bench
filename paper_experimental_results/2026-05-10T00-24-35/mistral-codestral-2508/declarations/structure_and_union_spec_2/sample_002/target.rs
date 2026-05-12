use std::mem;

fn main() {
    #[repr(C)]
    struct S {
        n: i32,
        d: [f64; 0],
    }

    #[repr(C)]
    struct SS {
        n: i32,
    }

    if mem::size_of::<S>() < mem::size_of::<SS>() {
        std::process::exit(1);
    }

    if mem::size_of_val(&S { n: 0, d: [] }) != mem::size_of::<S>() {
        std::process::exit(1);
    }

    let s1: Box<S> = unsafe {
        let layout = std::alloc::Layout::from_size_align_unchecked(
            mem::size_of::<S>() + 8 * mem::size_of::<f64>(),
            mem::align_of::<S>(),
        );
        let ptr = std::alloc::alloc(layout);
        Box::from_raw(ptr as *mut S)
    };

    let s2: Box<S> = unsafe {
        let layout = std::alloc::Layout::from_size_align_unchecked(
            mem::size_of::<S>() + 5 * mem::size_of::<f64>(),
            mem::align_of::<S>(),
        );
        let ptr = std::alloc::alloc(layout);
        Box::from_raw(ptr as *mut S)
    };

    if s1.d.as_ptr().is_null() || s2.d.as_ptr().is_null() {
        std::process::exit(1);
    }

    unsafe {
        *s1.d.as_ptr() = 42.0;
        *s2.d.as_ptr() = 24.0;
    }

    if unsafe { *s1.d.as_ptr() } != 42.0 || unsafe { *s2.d.as_ptr() } != 24.0 {
        std::process::exit(1);
    }

    std::mem::drop(s1);
    std::mem::drop(s2);
}