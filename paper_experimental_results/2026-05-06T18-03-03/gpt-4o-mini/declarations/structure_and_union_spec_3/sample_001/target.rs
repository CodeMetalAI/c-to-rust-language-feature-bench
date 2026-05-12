use std::alloc::{alloc, dealloc, Layout};
use std::mem::{self, MaybeUninit};

struct S {
    i: i32,
    a: [i32; 0], // flexible array member
}

fn main() -> i32 {
    // Check offsets
    if mem::offset_of!(S, i) != 0 {
        return 1;
    }

    if mem::size_of::<S>() != mem::size_of::<S>() {
        return 2;
    }

    // Allocate memory for S plus 4 additional integers
    let layout = Layout::from_size_align(mem::size_of::<S>() + 4 * mem::size_of::<i32>(), mem::align_of::<S>()).unwrap();
    let p = unsafe { alloc(layout) as *mut S };

    if p.is_null() {
        return 3;
    }

    unsafe {
        (*p).i = 7;
        let a_ptr = (*p).a.as_mut_ptr();
        a_ptr.write(11);
        a_ptr.add(3).write(22);

        if (*p).i != 7 || *a_ptr != 11 || *a_ptr.add(3) != 22 {
            dealloc(p as *mut u8, layout);
            return 4;
        }
    }

    unsafe {
        dealloc(p as *mut u8, layout);
    }
    
    0
}