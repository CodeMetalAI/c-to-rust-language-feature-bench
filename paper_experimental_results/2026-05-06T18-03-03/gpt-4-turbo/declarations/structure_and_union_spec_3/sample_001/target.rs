use std::alloc::{alloc, dealloc, Layout};
use std::mem;

struct S {
    i: i32,
    a: [i32],
}

fn main() {
    let layout = Layout::new::<i32>().extend(Layout::array::<i32>(4).unwrap()).unwrap().0.pad_to_align();
    let ptr = unsafe { alloc(layout) } as *mut i32;
    if ptr.is_null() {
        std::process::exit(3);
    }

    unsafe {
        *ptr = 7; // equivalent to p->i = 7;
        *ptr.add(1) = 11; // equivalent to p->a[0] = 11;
        *ptr.add(4) = 22; // equivalent to p->a[3] = 22;

        if *ptr != 7 || *ptr.add(1) != 11 || *ptr.add(4) != 22 {
            std::process::exit(4);
        }

        dealloc(ptr as *mut u8, layout);
    }
}