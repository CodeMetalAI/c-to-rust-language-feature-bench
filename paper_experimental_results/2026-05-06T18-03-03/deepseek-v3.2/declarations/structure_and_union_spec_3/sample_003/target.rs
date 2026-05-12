use std::alloc::{alloc, dealloc, Layout};
use std::mem::{size_of, align_of};

struct S {
    i: i32,
    a: [i32; 0],
}

fn main() {
    // Check offset of i (should be 0)
    if offset_of!(S, i) != 0 {
        std::process::exit(1);
    }

    // Check offset of a (should be size_of::<i32>() due to alignment)
    if offset_of!(S, a) != size_of::<i32>() {
        std::process::exit(2);
    }

    // Allocate memory for S plus 4 additional i32s
    let layout = Layout::new::<S>();
    let total_size = layout.size() + 4 * size_of::<i32>();
    let align = layout.align().max(align_of::<i32>());
    let layout = Layout::from_size_align(total_size, align).unwrap();

    let ptr = unsafe { alloc(layout) } as *mut S;
    if ptr.is_null() {
        std::process::exit(3);
    }

    unsafe {
        // Initialize the struct
        (*ptr).i = 7;
        
        // Get pointer to the flexible array part
        let a_ptr = (*ptr).a.as_ptr() as *mut i32;
        *a_ptr.offset(0) = 11;
        *a_ptr.offset(3) = 22;

        // Check values
        if (*ptr).i != 7 || *a_ptr.offset(0) != 11 || *a_ptr.offset(3) != 22 {
            dealloc(ptr as *mut u8, layout);
            std::process::exit(4);
        }

        dealloc(ptr as *mut u8, layout);
    }

    std::process::exit(0);
}

// Macro to compute field offset (similar to offsetof)
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {{
        let dummy = std::mem::MaybeUninit::<$ty>::uninit();
        let base = dummy.as_ptr();
        let field = unsafe { &(*base).$field as *const _ };
        (field as usize) - (base as usize)
    }};
}