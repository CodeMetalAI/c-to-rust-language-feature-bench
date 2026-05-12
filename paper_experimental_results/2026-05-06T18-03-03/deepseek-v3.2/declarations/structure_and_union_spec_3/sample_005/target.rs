use std::alloc::{alloc, dealloc, Layout};
use std::mem::{offset_of, size_of};

#[repr(C)]
struct S {
    i: i32,
    a: [i32; 0],
}

fn main() {
    if offset_of!(S, i) != 0 {
        std::process::exit(1);
    }

    if offset_of!(S, a) != size_of::<S>() {
        std::process::exit(2);
    }

    let total_size = size_of::<S>() + 4 * size_of::<i32>();
    let layout = Layout::from_size_align(total_size, std::mem::align_of::<S>())
        .expect("Invalid layout");

    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        std::process::exit(3);
    }

    let p = ptr as *mut S;
    unsafe {
        (*p).i = 7;
        let a_ptr = (*p).a.as_ptr() as *mut i32;
        *a_ptr.offset(0) = 11;
        *a_ptr.offset(3) = 22;
    }

    unsafe {
        if (*p).i != 7 || *((*p).a.as_ptr() as *const i32).offset(0) != 11 || *((*p).a.as_ptr() as *const i32).offset(3) != 22 {
            dealloc(ptr, layout);
            std::process::exit(4);
        }
    }

    unsafe {
        dealloc(ptr, layout);
    }
    std::process::exit(0);
}