use std::mem;

struct S {
    i: i32,
    a: [i32; 0],
}

fn main() {
    let mut p: Box<S>;

    if mem::offset_of!(S, i) != 0 {
        std::process::exit(1);
    }

    if mem::offset_of!(S, a) != mem::size_of::<S>() {
        std::process::exit(2);
    }

    let size = mem::size_of::<S>() + 4 * mem::size_of::<i32>();
    let layout = std::alloc::Layout::from_size_align(size, mem::align_of::<S>()).unwrap();
    unsafe {
        let ptr = std::alloc::alloc(layout) as *mut S;
        if ptr.is_null() {
            std::process::exit(3);
        }
        p = Box::from_raw(ptr);
    }

    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;

    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        std::process::exit(4);
    }

    drop(p);
    std::process::exit(0);
}