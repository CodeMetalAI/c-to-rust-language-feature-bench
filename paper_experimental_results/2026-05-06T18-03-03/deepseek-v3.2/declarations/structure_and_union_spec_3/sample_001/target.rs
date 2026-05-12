use std::mem::{offset_of, size_of};
use std::alloc::{alloc, dealloc, Layout};

struct S {
    i: i32,
    a: [i32; 0], // zero-sized array to represent flexible array member
}

fn main() {
    // Check offsets
    if offset_of!(S, i) != 0 {
        return;
    }
    if offset_of!(S, a) != size_of::<S>() {
        return;
    }

    // Allocate memory for S plus 4 extra integers
    let layout = Layout::new::<S>()
        .extend(Layout::new::<i32>().repeat(4).unwrap())
        .unwrap()
        .0;
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        return;
    }

    // Cast pointer to S* and write values
    let p = ptr as *mut S;
    unsafe {
        (*p).i = 7;
        // Access flexible array member by offsetting past S
        let a_ptr = (ptr as *mut i32).add(size_of::<S>() / size_of::<i32>());
        *a_ptr = 11;
        *a_ptr.add(3) = 22;
    }

    // Check values
    unsafe {
        if (*p).i != 7 || *((ptr as *mut i32).add(size_of::<S>() / size_of::<i32>())) != 11 ||
            *((ptr as *mut i32).add(size_of::<S>() / size_of::<i32>() + 3)) != 22 {
            dealloc(ptr, layout);
            return;
        }
    }

    unsafe { dealloc(ptr, layout) };
}