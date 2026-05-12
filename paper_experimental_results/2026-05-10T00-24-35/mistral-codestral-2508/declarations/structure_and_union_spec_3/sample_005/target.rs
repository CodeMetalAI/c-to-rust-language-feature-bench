use std::mem;

#[repr(C)]
struct S {
    i: i32,
    a: [i32; 0], // Zero-sized array to represent the flexible array member
}

fn main() {
    let mut p: *mut S;

    // Calculate the offset of `i` in `S`
    let i_offset = unsafe { &(*(0 as *const S)).i as *const i32 as usize };
    if i_offset != 0 {
        std::process::exit(1);
    }

    // Calculate the offset of `a` in `S`
    let a_offset = unsafe { &(*(0 as *const S)).a as *const [i32; 0] as usize };
    if a_offset != mem::size_of::<S>() {
        std::process::exit(2);
    }

    // Allocate memory for `S` plus 4 `i32`s
    p = unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(mem::size_of::<S>() + 4 * mem::size_of::<i32>(), 1).unwrap()) as *mut S };
    if p.is_null() {
        std::process::exit(3);
    }

    unsafe {
        (*p).i = 7;
        (*p).a.as_ptr().offset(0).write(11);
        (*p).a.as_ptr().offset(3).write(22);

        if (*p).i != 7 || (*p).a.as_ptr().offset(0).read() != 11 || (*p).a.as_ptr().offset(3).read() != 22 {
            std::process::exit(4);
        }
    }

    unsafe {
        std::alloc::dealloc(p as *mut u8, std::alloc::Layout::from_size_align(mem::.size_of::<S>() + 4 * mem::size_of::<i32>(), 1).unwrap());
    }
}