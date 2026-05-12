use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{addr_of, NonNull};

#[repr(C)]
struct Inner {
    i: i32,
}

#[repr(C)]
struct S {
    inner: Inner,
    a: [i32; 0],
}

fn main() {
    // Check offset of i
    let offset_i = unsafe { addr_of!((*(NonNull::dangling().as_ptr() as *const S)).inner.i) } as usize;
    if offset_i != 0 {
        return;
    }

    // Check offset of a
    let offset_a = unsafe { addr_of!((*(NonNull::dangling().as_ptr() as *const S)).a) } as usize;
    let size_s = std::mem::size_of::<S>();
    if offset_a != size_s {
        return;
    }

    // Allocate memory for S plus 4 ints
    let total_size = size_s + 4 * std::mem::size_of::<i32>();
    let layout = Layout::from_size_align(total_size, std::mem::align_of::<S>()).unwrap();
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        return;
    }

    // Cast to pointer to S
    let p = ptr as *mut S;

    // Initialize fields
    unsafe {
        (*p).inner.i = 7;
        let a_ptr = (*p).a.as_ptr() as *mut i32;
        a_ptr.write(11);
        a_ptr.add(3).write(22);
    }

    // Check values
    unsafe {
        if (*p).inner.i != 7 || (*p).a.as_ptr().read() != 11 || (*p).a.as_ptr().add(3).read() != 22 {
            dealloc(ptr, layout);
            return;
        }
    }

    // Free memory
    unsafe {
        dealloc(ptr, layout);
    }
}