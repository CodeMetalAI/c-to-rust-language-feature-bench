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
        mem::size_of::<S>() + mem::size_of::<f64>() * 8,
        mem::align_of::<S>(),
    )
    .unwrap();
    let ptr1 = unsafe { alloc(layout1) } as *mut S;

    let layout2 = Layout::from_size_align(
        mem::size_of::<S>() + mem::size_of::<f64>() * 5,
        mem::align_of::<S>(),
    )
    .unwrap();
    let ptr2 = unsafe { alloc(layout2) } as *mut S;

    if ptr1.is_null() || ptr2.is_null() {
        process::exit(1);
    }

    unsafe {
        (*ptr1).n = 0; // Not used in checks
        let d1_ptr = (*ptr1).d.as_mut_ptr();
        *d1_ptr = 42.0;

        (*ptr2).n = 0; // Not used in checks
        let d2_ptr = (*ptr2).d.as_mut_ptr();
        *d2_ptr = 24.0;

        if *d1_ptr != 42.0 || *d2_ptr != 24.0 {
            process::exit(1);
        }

        dealloc(ptr1 as *mut u8, layout1);
        dealloc(ptr2 as *mut u8, layout2);
    }

    process::exit(0);
}