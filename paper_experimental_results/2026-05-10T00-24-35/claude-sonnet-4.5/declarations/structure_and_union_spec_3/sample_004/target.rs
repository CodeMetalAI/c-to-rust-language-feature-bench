use std::alloc::{alloc, dealloc, Layout};
use std::mem::{offset_of, size_of};
use std::ptr;

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

    let layout = Layout::from_size_align(
        size_of::<S>() + 4 * size_of::<i32>(),
        std::mem::align_of::<S>()
    ).unwrap();
    
    let p = alloc(layout) as *mut S;
    if p.is_null() {
        std::process::exit(3);
    }

    ptr::write(&mut (*p).i, 7);
    ptr::write((*p).a.as_mut_ptr().add(0), 11);
    ptr::write((*p).a.as_mut_ptr().add(3), 22);

    if ptr::read(&(*p).i) != 7 
        || ptr::read((*p).a.as_ptr().add(0)) != 11 
        || ptr::read((*p).a.as_ptr().add(3)) != 22 {
        dealloc(p as *mut u8, layout);
        std::process::exit(4);
    }

    dealloc(p as *mut u8, layout);
    std::process::exit(0);
}