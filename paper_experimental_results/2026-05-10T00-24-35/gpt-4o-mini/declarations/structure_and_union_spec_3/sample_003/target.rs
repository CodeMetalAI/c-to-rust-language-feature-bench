use std::mem;

struct S {
    i: i32,
    a: [i32; 0], // Placeholder to emulate flexible array member
}

fn main() -> i32 {
    // Check offsets
    if mem::offset_of!(S, i) != 0 {
        return 1;
    }
    
    // Calculate size of S with a flexible array member
    let size_of_s = mem::size_of::<S>();
    if mem::offset_of!(S, a) != size_of_s {
        return 2;
    }

    // Allocate memory for S and additional integers
    let p = unsafe {
        let layout = std::alloc::Layout::from_size_align(size_of_s + 4 * mem::size_of::<i32>(), mem::align_of::<S>()).unwrap();
        let ptr = std::alloc::alloc(layout);
        if ptr.is_null() {
            return 3;
        }
        ptr as *mut S
    };

    unsafe {
        (*p).i = 7;
        (*p).a[0] = 11;
        (*p).a[3] = 22;

        if (*p).i != 7 || (*p).a[0] != 11 || (*p).a[3] != 22 {
            std::alloc::dealloc(p as *mut u8, std::alloc::Layout::from_size_align(size_of_s + 4 * mem::size_of::<i32>(), mem::align_of::<S>()).unwrap());
            return 4;
        }

        std::alloc::dealloc(p as *mut u8, std::alloc::Layout::from_size_align(size_of_s + 4 * mem::size_of::<i32>(), mem::align_of::<S>()).unwrap());
    }

    0
}