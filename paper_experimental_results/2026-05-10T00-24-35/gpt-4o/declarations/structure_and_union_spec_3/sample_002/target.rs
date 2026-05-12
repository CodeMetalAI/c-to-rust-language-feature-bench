use std::mem;
use std::alloc::{alloc, dealloc, Layout};

struct S {
    i: i32,
    a: [i32; 0],
}

fn main() {
    // Check offsets
    if mem::size_of::<i32>() != 0 {
        std::process::exit(1);
    }

    if mem::size_of::<S>() != mem::size_of::<i32>() {
        std::process::exit(2);
    }

    // Allocate memory for struct S with additional space for 4 integers
    let layout = Layout::new::<S>().extend(Layout::array::<i32>(4).unwrap()).unwrap().0;
    let p = unsafe { alloc(layout) as *mut S };
    if p.is_null() {
        std::process::exit(3);
    }

    unsafe {
        (*p).i = 7;
        let a_ptr = (*p).a.as_ptr() as *mut i32;
        a_ptr.add(0).write(11);
        a_ptr.add(3).write(22);

        if (*p).i != 7 || a_ptr.read() != 11 || a_ptr.add(3).read() != 22 {
            dealloc(p as *mut u8, layout);
            std::process::exit(4);
        }

        dealloc(p as *mut u8, layout);
    }

    std::process::exit(0);
}