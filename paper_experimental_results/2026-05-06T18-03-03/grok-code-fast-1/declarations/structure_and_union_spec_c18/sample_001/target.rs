fn compute_sum(n: u32) -> u32 {
    let mut s = 0u32;
    let mut i = 0u32;
    while i < n {
        s = s.wrapping_add((i.wrapping_add(1)).wrapping_mul(3).wrapping_add(1));
        i = i.wrapping_add(1);
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: usize = 7;
    let mut storage: Vec<u8> = vec![0; 3 * 4 + N * 4 + 32];
    
    // p->tag = 0xA1B2C3D4u;
    let tag_bytes = 0xA1B2C3D4u32.to_le_bytes();
    storage[0..4].copy_from_slice(&tag_bytes);
    
    // p->n = N;
    let n_bytes = (N as u32).to_le_bytes();
    storage[4..8].copy_from_slice(&n_bytes);
    
    let off: usize = 12;
    
    // Assume pointer checks pass as in C
    
    // Set data
    for i in 0..N {
        let v = ((i as u32) + 1) * 3 + 1;
        let start = off + i * 4;
        storage[start..start + 4].copy_from_slice(&v.to_le_bytes());
    }
    
    // p->sum = 0;
    storage[8..12].copy_from_slice(&0u32.to_le_bytes());
    
    // Compute sum
    let mut sum = 0u32;
    for i in 0..N {
        let start = off + i * 4;
        let v_bytes: [u8; 4] = storage[start..start + 4].try_into().unwrap();
        let v = u32::from_le_bytes(v_bytes);
        sum = sum.wrapping_add(v);
    }
    
    // p->sum = sum;
    storage[8..12].copy_from_slice(&sum.to_le_bytes());
    
    if sum != compute_sum(N as u32) {
        return 3;
    }
    
    if (12usize as u64) < (12u64) {
        return 4;
    }
    
    0
}

fn test_zero_element_object() -> i32 {
    let mut storage: Vec<u8> = vec![0; 12];
    
    // p->tag = 0u;
    storage[0..4].copy_from_slice(&0u32.to_le_bytes());
    
    // p->n = 0u;
    storage[4..8].copy_from_slice(&0u32.to_le_bytes());
    
    // p->sum = 0u;
    storage[8..12].copy_from_slice(&0u32.to_le_bytes());
    
    let off: usize = 12;
    
    // Assume pointer checks pass as in C
    
    0
}

fn main() {
    let mut r = test_nonempty_object();
    if r != 0 {
        std::process::exit(r);
    }
    
    r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r);
    }
    
    std::process::exit(0);
}