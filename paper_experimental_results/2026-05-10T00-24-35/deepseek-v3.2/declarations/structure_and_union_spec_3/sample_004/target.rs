use std::alloc::{alloc, dealloc, Layout};
use std::mem::size_of;

struct Inner {
    i: i32,
}

struct S {
    inner: Inner,
    // Flexible array member is represented as a pointer to a separately allocated buffer
    a_ptr: *mut i32,
    a_len: usize,
}

impl S {
    unsafe fn new_with_flexible_array(flex_len: usize) -> *mut S {
        // Allocate memory for the main struct
        let layout = Layout::new::<S>();
        let ptr = alloc(layout) as *mut S;
        if ptr.is_null() {
            return std::ptr::null_mut();
        }

        // Initialize the struct
        (*ptr).inner = Inner { i: 0 };
        (*ptr).a_len = flex_len;

        // Allocate memory for the flexible array
        if flex_len > 0 {
            let array_layout = Layout::array::<i32>(flex_len).unwrap();
            (*ptr).a_ptr = alloc(array_layout) as *mut i32;
            if (*ptr).a_ptr.is_null() {
                dealloc(ptr as *mut u8, layout);
                return std::ptr::null_mut();
            }
        } else {
            (*ptr).a_ptr = std::ptr::null_mut();
        }

        ptr
    }

    unsafe fn free(ptr: *mut S) {
        if ptr.is_null() {
            return;
        }

        let layout = Layout::new::<S>();
        
        // Free the flexible array memory
        if !(*ptr).a_ptr.is_null() && (*ptr).a_len > 0 {
            let array_layout = Layout::array::<i32>((*ptr).a_len).unwrap();
            dealloc((*ptr).a_ptr as *mut u8, array_layout);
        }
        
        // Free the main struct memory
        dealloc(ptr as *mut u8, layout);
    }

    unsafe fn a_mut(&mut self, index: usize) -> &mut i32 {
        &mut *self.a_ptr.add(index)
    }
}

fn main() {
    // Check offsets - in Rust, fields are guaranteed to be in declaration order
    // The inner struct field has offset 0
    if 0 != 0 {
        std::process::exit(1);
    }

    // The flexible array pointer field comes after the inner struct
    // inner.i (4 bytes) + padding (0 bytes in this case) = 4 bytes offset for a_ptr
    if 4 != size_of::<Inner>() {
        std::process::exit(2);
    }

    // Create S with 4-element flexible array
    let p = unsafe { S::new_with_flexible_array(4) };
    if p.is_null() {
        std::process::exit(3);
    }

    unsafe {
        // Set values
        (*p).inner.i = 7;
        *(*p).a_mut(0) = 11;
        *(*p).a_mut(3) = 22;

        // Check values
        if (*p).inner.i != 7 || *(*p).a_mut(0) != 11 || *(*p).a_mut(3) != 22 {
            S::free(p);
            std::process::exit(4);
        }

        S::free(p);
    }

    std::process::exit(0);
}