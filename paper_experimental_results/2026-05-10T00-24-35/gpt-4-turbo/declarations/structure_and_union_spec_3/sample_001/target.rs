use std::mem;
use std::ptr;

struct Inner {
    i: i32,
}

struct S {
    inner: Inner,
    a: [i32; 0], // Using flexible array member equivalent
}

fn main() {
    // Check offsets
    assert_eq!(mem::size_of::<Inner>(), 4);
    assert_eq!(mem::size_of::<S>(), 4);

    // Allocate memory for S and 4 i32 elements
    let layout = std::alloc::Layout::from_size_align(mem::size_of::<S>() + 4 * mem::size_of::<i32>(), mem::align_of::<S>()).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) as *mut S };
    if ptr.is_null() {
        std::process::exit(3);
    }
    
    // Initialize memory
    let p = unsafe {
        ptr::write(&mut (*ptr).inner.i, 7);
        let a_slice = std::slice::from_raw_parts_mut(&mut (*ptr).a as *mut i32 as *mut i32, 4);
        a_slice[0] = 11;
        a_slice[3] = 22;
        &mut *ptr
    };

    // Check values
    if p.inner.i != 7 || unsafe { (*ptr).a.as_ptr().read() } != 11 || unsafe { (*ptr).a.as_ptr().add(3).read() } != 22 {
        std::process::exit(4);
    }

    // Free memory
    unsafe {
        std::alloc::dealloc(ptr as *mut u8, layout);
    }
}