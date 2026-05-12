use std::mem;

struct S {
    i: i32,
    a: [i32; 0],
}

fn main() {
    let mut p: Box<S>;

    if mem::size_of::<S>() != mem::size_of::<i32>() {
        std::process::exit(1);
    }

    if mem::size_of::<S>() != mem::size_of::<i32>() {
        std::process::exit(2);
    }

    let size = mem::size_of::<S>() + 4 * mem::size_of::<i32>();
    let ptr = unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(size, mem::align_of::<S>()).unwrap()) };
    if ptr.is_null() {
        std::process::exit(3);
    }

    p = unsafe { Box::from_raw(ptr as *mut S) };

    p.i = 7;
    unsafe {
        *p.a.as_ptr().offset(0) = 11;
        *p.a.as_ptr().offset(3) = 22;
    }

    if p.i != 7 || unsafe { *p.a.as_ptr().offset(0) } != 11 || unsafe { *p.a.as_ptr().offset(3) } != 22 {
        std::process::exit(4);
    }

    drop(p);
    std::process::exit(0);
}