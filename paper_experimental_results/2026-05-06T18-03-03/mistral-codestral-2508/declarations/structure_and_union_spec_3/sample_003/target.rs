use std::mem;

struct S {
    i: i32,
    a: [i32; 0],
}

fn main() {
    let mut p: Box<S> = unsafe {
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

    if mem::offset_of!(S, i) != 0 {
        std::process::exit(1);
    }

    if mem::offset_of!(S, a) != mem::size_of::<S>() {
        std::process::exit(2);
    }

    unsafe {
        p.i = 7;
        let a_ptr = p.a.as_ptr() as *mut i32;
        *a_ptr.offset(0) = 11;
        *a_ptr.offset(3) = 22;
    }

    if p.i != 7 || unsafe { *p.a.as_ptr().offset(0) } != 11 || unsafe { *p.a.as_ptr().offset(3) } != 22 {
        std::process::exit(4);
    }

    drop(p);
    std::process::exit(0);
}