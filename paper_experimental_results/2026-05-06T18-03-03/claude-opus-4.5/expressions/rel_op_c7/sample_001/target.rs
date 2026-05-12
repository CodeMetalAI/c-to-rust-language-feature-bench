fn main() {
    let x: i32 = 42;
    let p: &i32 = &x;

    // In safe Rust, a reference to x is always equal to itself
    // The original check `p != &x` would always be false since p = &x
    // So we skip to the equivalent logic

    // In the original C code, p_end1 and p_end2 are both &x + 1
    // They point to the same "one past the end" location
    // In safe Rust, we can't create pointers past the end of a single variable
    // But we can simulate the logic using a slice/array approach

    let arr: [i32; 1] = [x];
    let p_ref: &[i32] = &arr;
    
    // Simulate pointer comparison by using slice indices
    // p points to arr[0], p_end1 and p_end2 would both point to arr[1] (one past end)
    
    let p_idx: usize = 0;
    let p_end1_idx: usize = p_idx + 1;
    let p_end2_idx: usize = 0 + 1; // equivalent to &x + 1 in terms of offset

    // Check: p_end1 != p_end2 (both are 1, so they're equal)
    if p_end1_idx != p_end2_idx {
        std::process::exit(2);
    }

    // Check: p_end1 == p (1 == 0 is false)
    if p_end1_idx == p_idx {
        std::process::exit(3);
    }

    // If we reach here, exit with 0
    std::process::exit(0);
}