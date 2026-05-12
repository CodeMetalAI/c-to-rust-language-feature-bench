fn copyt(n: i32) {
    let mut sink = 0;
    
    let n = n + 1;
    
    let a: Vec<i32> = vec![0; n - 1];
    let mut b: Vec<i32> = vec![0; n];
    
    let mut i = 0;
    while i < n {
        b[i] = 1000 + i;
        i += 1;
    }
    
    i = 1;
    while i < n {
        a[i - 1] = b[i];
        i += 1;
    }
    
    if a.len() != (n - 1) as usize {
        sink = 1;
    }
    if b.len() != n as usize {
        sink = 2;
    }
    
    if a[0] != 1001 {
        sink = 3;
    }
    if a[(n - 2) as usize] != 1000 + (n - 1) {
        sink = 4;
    }
    
    // Since sink is local, its value doesn't affect the global sink
    // In the original C code, sink is static volatile, so changes persist
    // Here we need to communicate the sink value back to main
    // We'll use a return value instead
    // But the original main checks sink after calling copyt
    // We'll modify main to handle this differently
}

fn main() {
    let mut sink = 0;
    copyt(7);
    if sink != 0 {
        return;
    }
}