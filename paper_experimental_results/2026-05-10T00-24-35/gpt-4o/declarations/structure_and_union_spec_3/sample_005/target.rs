use std::mem;

struct S {
    i: i32,
    a: [i32; 0],
}

fn main() -> i32 {
    if mem::size_of::<i32>() != 0 {
        return 1;
    }

    if mem::size_of::<S>() != mem::size_of::<i32>() {
        return 2;
    }

    let size = mem::size_of::<S>() + 4 * mem::size_of::<i32>();
    let layout = std::alloc::Layout::from_size_align(size, mem::align_of::<i32>()).unwrap();
    let p = unsafe { std::alloc::alloc(layout) } as *mut S;

    if p.is_null() {
        return 3;
    }

    unsafe {
        (*p).i = 7;
        let a_ptr = p.add(1) as *mut i32;
        *a_ptr.offset(0) = 11;
        *a_ptr.offset(3) = 22;

        if (*p).i != 7 || *a_ptr.offset(0) != 11 || *a_ptr.offset(3) != 22 {
            std::alloc::dealloc(p as *mut u8, layout);
            return 4;
        }

        std::alloc::dealloc(p as *mut u8, layout);
    }

    0
}