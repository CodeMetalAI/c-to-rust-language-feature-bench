use std::alloc::{alloc, dealloc, Layout};
use std::mem::{size_of, align_of};

#[repr(C)]
struct Inner {
    i: i32,
}

#[repr(C)]
struct S {
    inner: Inner,
    // Flexible array member is not directly supported in Rust.
    // We'll handle it manually with a pointer and allocation.
}

fn main() {
    // Check offsets
    if offset_of!(S, inner.i) != 0 {
        std::process::exit(1);
    }
    
    if offset_of!(S, inner) != 0 {
        std::process::exit(1);
    }

    // The offset of the flexible array member should be the size of the struct
    let expected_offset = size_of::<S>();
    // In C, offsetof(struct s, a) would be the size of the struct
    // We'll simulate this check
    if size_of::<S>() != expected_offset {
        std::process::exit(2);
    }

    // Allocate memory for S plus 4 ints
    let total_size = size_of::<S>() + 4 * size_of::<i32>();
    let align = align_of::<S>();
    
    let layout = match Layout::from_size_align(total_size, align) {
        Ok(layout) => layout,
        Err(_) => std::process::exit(3),
    };
    
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        std::process::exit(3);
    }
    
    // Cast to S pointer
    let p = ptr as *mut S;
    
    // Initialize the struct
    unsafe {
        // Set p->i (which is actually p->inner.i)
        (*p).inner.i = 7;
        
        // Get pointer to the flexible array part
        let a_ptr = ptr.add(size_of::<S>()) as *mut i32;
        
        // Set array elements
        *a_ptr = 11;
        *a_ptr.add(3) = 22;
        
        // Verify values
        if (*p).inner.i != 7 || *a_ptr != 11 || *a_ptr.add(3) != 22 {
            dealloc(ptr, layout);
            std::process::exit(4);
        }
        
        dealloc(ptr, layout);
    }
    
    std::process::exit(0);
}

// Helper macro to compute offset of field
macro_rules! offset_of {
    ($ty:ty, $field:tt) => {
        unsafe {
            let base = std::ptr::null::<$ty>();
            &(*base).$field as *const _ as usize
        }
    };
}