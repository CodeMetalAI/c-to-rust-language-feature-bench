use std::alloc::{alloc, dealloc, Layout};
use std::mem::size_of;

#[repr(C)]
struct SInner {
    i: i32,
}

#[repr(C)]
struct S {
    inner: SInner,
    a: [i32; 0],
}

fn main() {
    // Check offset of i (through inner struct)
    if (std::ptr::addr_of!((*(std::ptr::null::<S>())).inner.i) as usize) != 0 {
        std::process::exit(1);
    }

    // Check offset of flexible array member
    if (std::ptr::addr_of!((*(std::ptr::null::<S>())).a) as usize) != size_of::<S>() {
        std::process::exit(2);
    }

    // Allocate memory for struct + 4 integers
    let layout = Layout::from_size_align(
        size_of::<S>() + 4 * size_of::<i32>(),
        std::mem::align_of::<S>(),
    )
    .unwrap();

    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        std::process::exit(3);
    }

    // Cast to pointer to S
    let p = ptr as *mut S;
    
    // Initialize fields
    unsafe {
        (*p).inner.i = 7;
        
        // Get pointer to the flexible array part (after the struct)
        let a_ptr = ptr.add(size_of::<S>()) as *mut i32;
        *a_ptr.offset(0) = 11;
        *a_ptr.offset(3) = 22;
        
        // Verify values
        if (*p).inner.i != 7 || *a_ptr.offset(0) != 11 || *a_ptr.offset(3) != 22 {
            dealloc(ptr, layout);
            std::process::exit(4);
        }
        
        dealloc(ptr, layout);
    }
    
    std::process::exit(0);
}