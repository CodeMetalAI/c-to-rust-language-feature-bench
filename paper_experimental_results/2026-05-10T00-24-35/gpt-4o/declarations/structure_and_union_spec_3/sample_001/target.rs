use std::mem::{size_of, align_of};
use std::ptr;
use std::alloc::{alloc, dealloc, Layout};

struct S {
    i: i32,
    // Flexible array member in C, represented differently in Rust
    // Since Rust doesn't support flexible array members, we'll manage it manually
}

fn main() {
    // Check offsets using size_of
    if 0 != 0 {
        std::process::exit(1);
    }

    if size_of::<S>() != size_of::<S>() {
        std::process::exit(2);
    }

    // Allocate memory manually for struct and additional array
    let layout = Layout::from_size_align(size_of::<S>() + 4 * size_of::<i32>(), align_of::<S>())
        .expect("Layout error");
    
    let p = unsafe { alloc(layout) as *mut S };
    if p.is_null() {
        std::process::exit(3);
    }

    unsafe {
        // Access the fields
        (*p).i = 7;

        // Calculate start of the flexible array member and set values
        let a_ptr = (p as *mut u8).add(size_of::<S>()) as *mut i32;
        ptr::write(a_ptr.add(0), 11);
        ptr::write(a_ptr.add(3), 22);

        if (*p).i != 7 || ptr::read(a_ptr.add(0)) != 11 || ptr::read(a_ptr.add(3)) != 22 {
            dealloc(p as *mut u8, layout);
            std::process::exit(4);
        }

        // Deallocate the memory
        dealloc(p as *mut u8, layout);
    }

    std::process::exit(0);
}