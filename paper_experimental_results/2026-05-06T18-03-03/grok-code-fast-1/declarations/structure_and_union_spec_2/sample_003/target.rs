use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::process;

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
    if mem::size_of::<S>() < mem::size_of::<Ss>() {
        process::exit(1);
    }
    if mem::offset_of!(S, d) != mem::size_of::<S>() {
        process::exit(1);
    }
    let layout1 = Layout::from_size_align(
        mem::size_of::<S>() + 8 * mem::size_of::<f64>(),
        mem::align_of::<f64>(),
    )
    .unwrap();
    let ptr1 = unsafe { alloc(layout1) as *mut S };
    if ptr1.is_null() {
        process::exit(1);
    }
    let layout2 = Layout::from_size_align(
        mem::size_of::<S>() + 5 * mem::size_of::<f64>(),
        mem::align_of::<f64>(),
    )
    .unwrap();
    let ptr2 = unsafe { alloc(layout2) as *mut S };
    if ptr2.is_null() {
        unsafe { dealloc(ptr1 as *mut u8, layout1) };
        process::exit(1);
    }
    unsafe {
        (*ptr1).d.as_mut_ptr().write(42.0);
        (*ptr2).d.as_mut_ptr().write(24.0);
        if *(*ptr1).d.as_ptr() != 42.0 || *(*ptr2).d.as_ptr() != 24.0 {
            dealloc(ptr1 as *mut u8, layout1);
            dealloc(ptr2 as *mut u8, layout2);
            process::exit(1);
        }
        dealloc(ptr1 as *mut u8, layout1);
        dealloc(ptr2 as *mut u8, layout2);
    }
    process::exit(0);
}