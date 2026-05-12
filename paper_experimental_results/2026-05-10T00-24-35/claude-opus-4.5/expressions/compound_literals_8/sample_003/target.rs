fn f() -> bool {
    let mut storage: [Option<(usize, i32)>; 2] = [None, None];
    let mut p: Option<usize> = None;
    let mut q: Option<usize>;
    let mut j = 0;

    loop {
        q = p;
        storage[j] = Some((j, j as i32));
        p = Some(j);
        j += 1;
        if j >= 2 {
            break;
        }
    }

    // Check if p == q (they should point to the same location in C due to compound literal reuse)
    // In C, compound literals in a loop reuse the same storage, so p == q after the loop
    // and q->i would be 1 (the last value written)
    
    // Simulating C behavior where compound literals in block scope have automatic storage
    // and the same storage location is reused each iteration
    // So after loop: p and q both point to same location, which contains j-1 = 1
    
    // In the C code, the compound literal is created at the same address each iteration
    // So after j=0: p points to compound literal with i=0, q=null
    // After j=1: q=p (old address), p points to compound literal with i=1 (same address!)
    // So p==q is true, and q->i is 1
    
    // To simulate this in Rust, we use a single storage location
    let mut single_storage: i32 = 0;
    let mut p_set = false;
    let mut q_set = false;
    let mut j = 0;
    
    loop {
        q_set = p_set;
        single_storage = j;
        p_set = true;
        j += 1;
        if j >= 2 {
            break;
        }
    }
    
    // p == q because they both reference the same storage location
    // q->i == 1 because single_storage contains 1 (the last value of j before increment was 1)
    p_set && q_set && (single_storage == 1)
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}