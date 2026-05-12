use std::alloc::{alloc, dealloc, Layout};
use std::mem::{offset_of, size_of};
use std::ptr;

#[repr(C)]
struct S {
    i: i32,
}

fn main() {
    if offset_of!(S, i) != 0 {
        std::process::exit(1);
    }

    let base_size = size_of::<S>();
    if base_size != size_of::<S>() {
        std::process::exit(2);
    }

    let total_size = base_size + 4 * size_of::<i32>();
    let align = std::mem::align_of::<S>();
    
    let layout = Layout::from_size_align(total_size, align).unwrap();
    let p = unsafe { alloc(layout) as *mut S };
    
    if p.is_null() {
        std::process::exit(3);
    }

    unsafe {
        (*p).i = 7;
        
        let a_ptr = (p as *mut u8).add(base_size) as *mut i32;
        *a_ptr.add(0) = 11;
        *a_ptr.add(3) = 22;

        if (*p).i != 7 || *a_ptr.add(0) != 11 || *a_ptr.add(3) != 22 {
            dealloc(p as *mut u8, layout);
            std::process::exit(4);
        }

        dealloc(p as *mut u8, layout);
    }

    std::process::exit(0);
}