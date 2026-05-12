use std::mem;

struct S {
    i: i32,
    a: [i32; 0], // flexible array member emulation
}

fn main() -> i32 {
    // Check offsets
    if mem::offset_of!(S, i) != 0 {
        return 1;
    }

    if mem::offset_of!(S, a) != mem::size_of::<S>() {
        return 2;
    }

    // Allocate memory for the struct and the array
    let size = mem::size_of::<S>() + 4 * mem::size_of::<i32>();
    let p = unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(size, 1).unwrap()) as *mut S };

    if p.is_null() {
        return 3;
    }

    unsafe {
        (*p).i = 7;
        let a = std::slice::from_raw_parts_mut((*p).a.as_mut_ptr(), 4);
        a[0] = 11;
        a[3] = 22;

        if (*p).i != 7 || a[0] != 11 || a[3] != 22 {
            std::alloc::dealloc(p as *mut u8, std::alloc::Layout::from_size_align(size, 1).unwrap());
            return 4;
        }

        // Deallocate memory
        std::alloc::dealloc(p as *mut u8, std::alloc::Layout::from_size_align(size, 1).unwrap());
    }

    return 0;
}