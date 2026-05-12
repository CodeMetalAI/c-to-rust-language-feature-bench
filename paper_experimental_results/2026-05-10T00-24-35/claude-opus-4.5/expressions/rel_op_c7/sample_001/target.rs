fn main() {
    let x: i32 = 42;
    let p: &i32 = &x;

    // In safe Rust, we can't directly compare pointer addresses the same way,
    // but we can use std::ptr::eq to compare references for pointer equality
    if !std::ptr::eq(p, &x) {
        std::process::exit(1);
    }

    // In safe Rust, we can't do pointer arithmetic like p + 1
    // However, we can simulate the behavior using slices
    // The original code checks if (p + 1) == (&x + 1), which would always be true
    // since p == &x, so p + 1 == &x + 1
    
    // We can use a slice to get similar semantics
    let arr: [i32; 1] = [x];
    let p_slice: &[i32] = &arr;
    
    // Get pointers to one-past-the-end
    let p_end1: *const i32 = p_slice.as_ptr_range().end;
    let p_end2: *const i32 = arr.as_ptr_range().end;

    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    // Check that p_end1 != p (one-past-end is not equal to start)
    let p_start: *const i32 = p_slice.as_ptr();
    if p_end1 == p_start {
        std::process::exit(3);
    }

    std::process::exit(0);
}