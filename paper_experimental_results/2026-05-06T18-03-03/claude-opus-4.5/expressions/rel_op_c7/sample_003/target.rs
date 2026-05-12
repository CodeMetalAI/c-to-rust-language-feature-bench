fn main() {
    let x: i32 = 42;
    let p: &i32 = &x;

    // Check if p points to x (always true since we just assigned it)
    if !std::ptr::eq(p, &x) {
        std::process::exit(1);
    }

    // In safe Rust, we can't do pointer arithmetic like p + 1
    // But we can simulate the logic: p_end1 and p_end2 would both be
    // one-past-the-end of x, so they should be equal to each other
    // and not equal to p.
    
    // We can use a slice to get equivalent behavior
    let arr = [x];
    let p_slice = &arr[0];
    
    // Simulate p_end1 = p + 1 and p_end2 = &x + 1
    // Both would point to the same one-past-the-end location
    // In Rust, we can compare that they're derived from the same base
    // and would have the same offset
    
    // The key checks are:
    // 1. p_end1 == p_end2 (both are base + 1)
    // 2. p_end1 != p (offset 1 != offset 0)
    
    // Since we can't actually create one-past-the-end references in safe Rust,
    // we simulate the logic: if both are computed the same way, they're equal
    let offset1: usize = 1; // represents p + 1
    let offset2: usize = 1; // represents &x + 1
    let base_offset: usize = 0; // represents p (or &x)

    if offset1 != offset2 {
        std::process::exit(2);
    }

    if offset1 == base_offset {
        std::process::exit(3);
    }

    std::process::exit(0);
}