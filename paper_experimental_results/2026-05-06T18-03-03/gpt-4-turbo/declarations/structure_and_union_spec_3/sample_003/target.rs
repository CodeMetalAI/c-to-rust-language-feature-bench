use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

struct S {
    i: i32,
    a: [i32],
}

fn main() {
    // Check the offset of `i` in `S`
    if mem::offset_of!(S, i) != 0 {
        std::process::exit(1);
    }

    // Check the offset of `a` in `S`
    if mem::offset_of!(S, a) != mem::size_of::<i32>() {
        std::process::exit(2);
    }

    // Allocate memory for `S` and 4 additional i32s
    let layout = Layout::new::<i32>().repeat(1 + 4).unwrap().0;
    let ptr = unsafe { alloc(layout) } as *mut i32;
    if ptr.is_null() {
        std::process::exit(3);
    }

    // Initialize `i` and `a`
    unsafe {
        ptr::write(ptr, 7); // S.i
        ptr::write(ptr.add(1), 11); // S.a[0]
        ptr::write(ptr.add(4), 22); // S.a[3]
    }

    // Check values
    let i = unsafe { ptr::read(ptr) };
    let a0 = unsafe { ptr::read(ptr.add(1)) };
    let a3 = unsafe { ptr::read(ptr.add(4)) };
    if i != 7 || a0 != 11 || a3 != 22 {
        std::process::exit(4);
    }

    // Free memory
    unsafe {
        dealloc(ptr as *mut u8, layout);
    }
}