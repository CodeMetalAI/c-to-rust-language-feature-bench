fn main() {
    let x: i32 = 42;
    let p: &i32 = &x;

    // In safe Rust, we can't directly compare pointers the same way as C
    // but we can check if references point to the same location
    if !std::ptr::eq(p, &x) {
        std::process::exit(1);
    }

    // In safe Rust, we can't do pointer arithmetic like p + 1
    // However, we can simulate the logic using slices
    let arr: [i32; 1] = [x];
    let p_slice: &[i32] = &arr;
    
    // Get "one past the end" indices - in Rust we work with indices/slices
    // The original C code checks if two "one past end" pointers are equal
    // and if "one past end" is different from the start
    
    // Simulate with slice indices
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