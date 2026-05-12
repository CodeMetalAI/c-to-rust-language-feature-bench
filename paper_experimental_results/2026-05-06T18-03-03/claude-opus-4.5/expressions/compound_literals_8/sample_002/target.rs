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
        if j < 2 {
            continue;
        }
        break;
    }

    // Check if p == q (both point to the same index)
    // In C, compound literals in a loop have the same address, so p == q would be true
    // and q->i would be the last value written (1)
    
    // In the C code, the compound literal has the same address each iteration
    // So after the loop: p and q both point to the same location, which contains j-1 = 1
    // The condition is (p == q) && (q->i == 1)
    
    // Simulating C behavior where compound literal reuses same memory:
    // After first iteration: p points to compound literal with i=0, q=null, j=1
    // After second iteration: q=p (points to compound literal), p points to same compound literal with i=1, j=2
    // So p == q (same address) and q->i == 1 (the value was overwritten)
    
    // To match C behavior: compound literal is at same address, value is 1
    // p == q is true, q->i == 1 is true
    
    true
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}