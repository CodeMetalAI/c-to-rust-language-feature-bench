use std::alloc::{alloc, dealloc, Layout};
use std::mem::{size_of, offset_of};

struct S {
    i: i32,
    a: [i32; 0], // flexible array member
}

fn main() -> i32 {
    // Check offsets
    if offset_of!(S, i) != 0 {
        return 1;
    }

    if offset_of!(S, a) != size_of::<S>() {
        return 2;
    }

    // Allocate memory for struct S and 4 additional integers
    let layout = Layout::from_size_align(size_of::<S>() + 4 * size_of::<i32>(), std::mem::align_of::<S>()).unwrap();
    let p = unsafe { alloc(layout) as *mut S };

    if p.is_null() {
        return 3;
    }

    unsafe {
        (*p).i = 7;
        (*p).a[0] = 11;
        (*p).a[3] = 22;

        if (*p).i != 7 || (*p).a[0] != 11 || (*p).a[3] != 22 {
            dealloc(p as *mut u8, layout);
            return 4;
        }

        dealloc(p as *mut u8, layout);
    }

    0
}