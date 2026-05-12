use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

fn main() {
    // In Rust, we cannot directly define a flexible array member like in C.
    // Instead, we simulate it using a custom allocation and a raw slice.
    struct S {
        n: i32,
        // The flexible array member is not stored here directly.
    }

    struct SS {
        n: i32,
    }

    // Check sizes and offsets
    if std::mem::size_of::<S>() < std::mem::size_of::<SS>() {
        std::process::exit(1);
    }

    // In C, offsetof(struct s, d) equals sizeof(struct s) because d is a flexible array member.
    // In Rust, we cannot compute offset of a non-existent field, so we just check that
    // the size of S is at least the offset where d would start (which is just after n).
    // Since S only contains n, the size of S is the size of n plus padding.
    // The offset of d in C is after n, so we verify that the size of S is at least
    // the offset of the hypothetical d (which is the size of n plus any padding).
    // In practice, we just check that there is no extra padding before the flexible array.
    // For simplicity, we assume the C behavior: the flexible array starts right after n.
    // We'll check that the size of S is equal to the offset of the first element of d.
    // Since we can't have a field d, we'll compute the offset manually.
    let offset_d = std::mem::size_of::<i32>();
    // The size of S might include padding after n, but in C the flexible array starts
    // after any padding. However, the C code checks that offsetof(struct s, d) == sizeof(struct s).
    // That means there is no padding after n. So we check that the size of S equals the size of i32.
    if std::mem::size_of::<S>() != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }

    // Allocate memory for S with extra space for the double array.
    // We'll allocate a layout that includes space for S and the doubles.
    // We'll treat the allocated memory as a pointer to S, and then access the doubles
    // as a slice starting right after S.

    // For s1: 8 doubles
    let s1_size = std::mem::size_of::<S>() + 8 * std::mem::size_of::<f64>();
    let layout1 = Layout::from_size_align(s1_size, std::mem::align_of::<S>()).unwrap();
    let s1_ptr = unsafe { alloc(layout1) };
    if s1_ptr.is_null() {
        std::process::exit(1);
    }

    // For s2: 5 doubles
    let s2_size = std::mem::size_of::<S>() + 5 * std::mem::size_of::<f64>();
    let layout2 = Layout::from_size_align(s2_size, std::mem::align_of::<S>()).unwrap();
    let s2_ptr = unsafe { alloc(layout2) };
    if s2_ptr.is_null() {
        unsafe { dealloc(s1_ptr, layout1) };
        std::process::exit(1);
    }

    // Initialize the n field (optional, but we can set it to 0).
    unsafe {
        (s1_ptr as *mut S).write(S { n: 0 });
        (s2_ptr as *mut S).write(S { n: 0 });
    }

    // Access the double array: it starts right after S.
    let s1_doubles = unsafe {
        std::slice::from_raw_parts_mut(
            s1_ptr.add(std::mem::size_of::<S>()) as *mut f64,
            8,
        )
    };
    let s2_doubles = unsafe {
        std::slice::from_raw_parts_mut(
            s2_ptr.add(std::mem::size_of::<S>()) as *mut f64,
            5,
        )
    };

    s1_doubles[0] = 42.0;
    s2_doubles[0] = 24.0;

    if s1_doubles[0] != 42.0 || s2_doubles[0] != 24.0 {
        unsafe {
            dealloc(s1_ptr, layout1);
            dealloc(s2_ptr, layout2);
        }
        std::process::exit(1);
    }

    unsafe {
        dealloc(s1_ptr, layout1);
        dealloc(s2_ptr, layout2);
    }

    std::process::exit(0);
}