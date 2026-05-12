fn main() {
    let x: i32 = 42;
    let p: &i32 = &x;

    // In safe Rust, a reference to x is always equal to itself
    // The original check `p != &x` would always be false since p = &x
    // So we skip to the equivalent logic

    // In the original C code:
    // - p_end1 = p + 1 (pointer arithmetic)
    // - p_end2 = &x + 1 (pointer arithmetic)
    // Both point to one-past-the-end of x, so they're equal
    // p_end1 != p_end2 is false, so we don't return 2

    // p_end1 == p would compare one-past-end with start, which is false
    // So we don't return 3

    // In safe Rust, we can simulate this with slice references
    let arr = [x];
    let p_ref: &i32 = &arr[0];
    
    // Check if reference to x equals itself (always true, so condition is false)
    if std::ptr::eq(p, &x) == false {
        std::process::exit(1);
    }

    // For the pointer arithmetic checks, we use slice indices
    // p_end1 and p_end2 both point to index 1 (one past the single element)
    let p_idx: usize = 0;
    let p_end1_idx: usize = p_idx + 1;
    let p_end2_idx: usize = 0 + 1;

    if p_end1_idx != p_end2_idx {
        std::process::exit(2);
    }

    if p_end1_idx == p_idx {
        std::process::exit(3);
    }

    std::process::exit(0);
}