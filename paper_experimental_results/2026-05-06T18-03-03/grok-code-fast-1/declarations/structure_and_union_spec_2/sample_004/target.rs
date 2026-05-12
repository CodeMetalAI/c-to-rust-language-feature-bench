use std::alloc::{alloc, dealloc, Layout};
use std::mem::{size_of, offset_of, align_of};
use std::ptr;

#[repr(C)]
struct S {
    n: i32,
    d: [f64; 0],
}

#[repr(C)]
struct Ss {
    n: i32,
}

fn main() {
    if size_of::<S>() < size_of::<Ss>() {
        std::process::exit(1);
    }

    if offset_of!(S, d) != size_of::<S>() {
        std::process::exit(1);
    }

    let layout1 = Layout::from_size_align(
        size_of::<S>() + 8 * size_of::<f64>(),
        align_of::<S>(),
    )
    .unwrap();
    let ptr1 = unsafe { alloc(layout1) } as *mut S;

    let layout2 = Layout::from_size_align(
        size_of::<S>() + 5 * size_of::<f64>(),
        align_of::<S>(),
    )
    .unwrap();
    let ptr2 = unsafe { alloc(layout2) } as *mut S;

    if ptr1.is_null() || ptr2.is_null() {
        std::process::exit(1);
    }

    unsafe {
        ptr::write((*ptr1).d.as_mut_ptr(), 42.0);
        ptr::write((*ptr2).d.as_mut_ptr(), 24.0);

        if ptr::read((*ptr1).d.as_ptr()) != 42.0 || ptr::read((*ptr2).d.as_ptr()) != 24.0 {
            std::process::exit(1);
        }

        dealloc(ptr1 as *mut u8, layout1);
        dealloc(ptr2 as *mut u8, layout2);
    }

    std::process::exit(0);
}