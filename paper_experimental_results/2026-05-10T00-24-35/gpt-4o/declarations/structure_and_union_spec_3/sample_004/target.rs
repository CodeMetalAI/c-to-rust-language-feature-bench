use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

struct S {
    i: i32,
    // This will hold a pointer to the dynamically allocated array.
    a: *mut i32,
}

impl S {
    fn new(extra_elements: usize) -> Option<Box<S>> {
        // Calculate the total size: size of `S` + size for the extra elements
        let layout = Layout::new::<S>();
        let total_size = layout.size() + extra_elements * std::mem::size_of::<i32>();

        // Create a layout for the whole allocation
        let layout = Layout::from_size_align(total_size, layout.align()).ok()?;
        
        // Allocate the memory
        let raw_ptr = unsafe { alloc(layout) };
        if raw_ptr.is_null() {
            return None;
        }

        let s_ptr = raw_ptr as *mut S;

        unsafe {
            // Initialize `i`
            ptr::write(&mut (*s_ptr).i, 0);
            // Calculate the pointer to the array
            let a_offset = std::mem::size_of::<S>();
            let a_ptr = raw_ptr.add(a_offset) as *mut i32;
            (*s_ptr).a = a_ptr;
        }

        Some(unsafe { Box::from_raw(s_ptr) })
    }
}

impl Drop for S {
    fn drop(&mut self) {
        // Deallocate the memory when the struct is dropped.
        let layout = Layout::new::<S>();
        let total_size = layout.size() + 4 * std::mem::size_of::<i32>();
        let layout = Layout::from_size_align(total_size, layout.align()).unwrap();

        unsafe {
            dealloc(self as *mut S as *mut u8, layout);
        }
    }
}

fn main() -> i32 {
    let s_layout = Layout::new::<S>();
    
    if s_layout.align() != 4 {
        return 1;
    }

    if s_layout.size() != std::mem::size_of::<S>() {
        return 2;
    }

    let p = S::new(4);
    if p.is_none() {
        return 3;
    }

    let mut p = p.unwrap();

    p.i = 7;
    unsafe {
        ptr::write(p.a, 11);
        ptr::write(p.a.add(3), 22);
    }

    if p.i != 7 || unsafe { ptr::read(p.a) } != 11 || unsafe { ptr::read(p.a.add(3)) } != 22 {
        return 4;
    }

    0
}