use std::alloc::{Layout, alloc, dealloc};
use std::mem;

struct SInner {
    i: i32,
}

struct S {
    inner: SInner,
    // Flexible array member is represented as a slice pointer and manual allocation
    a_ptr: *mut i32,
    a_len: usize,
}

impl S {
    unsafe fn new_with_capacity(capacity: usize) -> *mut Self {
        // Calculate total size: size of S without the flexible array part + capacity * size of i32
        let base_layout = Layout::new::<SInner>();
        let array_layout = Layout::array::<i32>(capacity).unwrap();
        let (total_layout, array_offset) = base_layout.extend(array_layout).unwrap();

        // Allocate memory for the entire structure
        let ptr = alloc(total_layout) as *mut u8;

        // Cast to S pointer
        let s_ptr = ptr as *mut S;

        // Initialize the fields
        (*s_ptr).inner = SInner { i: 0 };
        (*s_ptr).a_ptr = ptr.add(array_offset) as *mut i32;
        (*s_ptr).a_len = capacity;

        s_ptr
    }

    unsafe fn free(ptr: *mut Self) {
        if ptr.is_null() {
            return;
        }

        // Calculate the layout to deallocate
        let base_layout = Layout::new::<SInner>();
        let array_layout = Layout::array::<i32>((*ptr).a_len).unwrap();
        let (total_layout, _) = base_layout.extend(array_layout).unwrap();

        dealloc(ptr as *mut u8, total_layout);
    }

    fn a(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(self.a_ptr, self.a_len) }
    }

    fn a_mut(&mut self) -> &mut [i32] {
        unsafe { std::slice::from_raw_parts_mut(self.a_ptr, self.a_len) }
    }
}

fn main() {
    // Check offset of i (which is now inside inner struct)
    let offset_i = mem::offset_of!(SInner, i);
    if offset_i != 0 {
        std::process::exit(1);
    }

    // Check offset of a_ptr (should be after inner)
    let offset_a_ptr = mem::offset_of!(S, a_ptr);
    if offset_a_ptr != mem::size_of::<SInner>() {
        std::process::exit(2);
    }

    // Allocate S with capacity for 4 ints
    let capacity = 4;
    let p = unsafe { S::new_with_capacity(capacity) };
    if p.is_null() {
        std::process::exit(3);
    }

    unsafe {
        // Set values
        (*p).inner.i = 7;
        (*p).a_mut()[0] = 11;
        (*p).a_mut()[3] = 22;

        // Check values
        if (*p).inner.i != 7 || (*p).a()[0] != 11 || (*p).a()[3] != 22 {
            S::free(p);
            std::process::exit(4);
        }

        // Free memory
        S::free(p);
    }

    std::process::exit(0);
}