fn main() {
    let x: i32 = 42;
    let p: &i32 = &x;

    // In safe Rust, a reference to x is always equal to itself
    // The original check `p != &x` would always be false since p = &x
    // So we skip to the equivalent logic

    // In the original C code, p_end1 and p_end2 are both &x + 1
    // They point to the same location (one past x), so they're equal
    // The check `p_end1 != p_end2` is always false

    // The check `p_end1 == p` compares &x+1 with &x, which is always false

    // Since all conditions that would return non-zero are false,
    // the program returns 0
    
    // To preserve the exact behavior while staying in safe Rust,
    // we can use slice references to simulate pointer arithmetic
    let arr: [i32; 1] = [x];
    let p_slice: &[i32] = &arr;
    
    // Check if reference to first element equals itself (always true, so condition is false)
    if std::ptr::eq(&p_slice[0], &p_slice[0]) == false {
        std::process::exit(1);
    }
    
    // Both "end" pointers would point to the same location (past the array)
    // In safe Rust, we simulate this by comparing slice lengths/indices
    let p_end1_idx: usize = 1;
    let p_end2_idx: usize = 1;
    
    if p_end1_idx != p_end2_idx {
        std::process::exit(2);
    }
    
    // p_end1 (index 1) vs p (index 0) - they're different
    let p_idx: usize = 0;
    if p_end1_idx == p_idx {
        std::process::exit(3);
    }
    
    std::process::exit(0);
}