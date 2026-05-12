use std::mem;

struct S {
    i: i32,
    a: [i32; 0], // Placeholder for flexible array member
}

fn main() -> i32 {
    // Check offset of i
    if mem::offset_of!(S, i) != 0 {
        return 1;
    }

    // Check offset of a
    let size_of_s = mem::size_of::<S>();
    if mem::offset_of!(S, a) != size_of_s {
        return 2;
    }

    // Allocate memory for S plus space for 4 integers
    let layout = std::alloc::Layout::from_size_align(size_of_s + 4 * mem::size_of::<i32>(), mem::align_of::<i32>()).unwrap();
    let p = unsafe { std::alloc::alloc(layout) as *mut S };

    if p.is_null() {
        return 3;
    }

    unsafe {
        (*p).i = 7;
        let a_ptr = (*p).a.as_mut_ptr();
        *a_ptr.add(0) = 11;
        *a_ptr.add(3) = 22;

        if (*p).i != 7 || *a_ptr.add(0) != 11 || *a_ptr.add(3) != 22 {
            std::alloc::dealloc(p as *mut u8, layout);
            return 4;
        }
    }

    unsafe {
        std::alloc::dealloc(p as *mut u8, layout);
    }

    return 0;
}