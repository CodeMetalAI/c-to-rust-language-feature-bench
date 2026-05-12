use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

#[repr(C)]
struct S {
    i: i32,
    a: [i32; 0],
}

fn main() -> i32 {
    if mem::offset_of!(S, i) != 0 {
        return 1;
    }

    if mem::offset_of!(S, a) != mem::size_of::<S>() {
        return 2;
    }

    let layout = Layout::from_size_align(
        mem::size_of::<S>() + 4 * mem::size_of::<i32>(),
        mem::align_of::<S>(),
    ).unwrap();
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        return 3;
    }

    let p = unsafe { &mut *(ptr as *mut S) };
    p.i = 7;

    let a_ptr = unsafe { ptr.add(mem::size_of::<S>()) as *mut i32 };
    unsafe {
        *a_ptr.add(0) = 11;
        *a_ptr.add(3) = 22;
    }

    if p.i != 7 || unsafe { *a_ptr.add(0) } != 11 || unsafe { *a_ptr.add(3) } != 22 {
        unsafe { dealloc(ptr, layout) };
        return 4;
    }

    unsafe { dealloc(ptr, layout) };
    0
}