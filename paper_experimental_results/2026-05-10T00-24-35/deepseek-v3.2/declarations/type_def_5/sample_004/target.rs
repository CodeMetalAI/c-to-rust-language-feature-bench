fn copyt(n: usize) {
    static mut SINK: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
    
    let n_plus_one = n + 1;
    
    let mut a: Vec<i32> = vec![0; n];
    let mut b: Vec<i32> = vec![0; n_plus_one];
    
    let mut i = 0;
    while i < n_plus_one {
        b[i] = 1000 + i as i32;
        i += 1;
    }
    
    i = 1;
    while i < n_plus_one {
        a[i - 1] = b[i];
        i += 1;
    }
    
    if a.len() != n {
        SINK.store(1, std::sync::atomic::Ordering::SeqCst);
    }
    if b.len() != n_plus_one {
        SINK.store(2, std::sync::atomic::Ordering::SeqCst);
    }
    
    if a[0] != 1001 {
        SINK.store(3, std::sync::atomic::Ordering::SeqCst);
    }
    if a[n - 1] != 1000 + (n_plus_one - 1) as i32 {
        SINK.store(4, std::sync::atomic::Ordering::SeqCst);
    }
}

fn main() {
    static mut SINK: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
    
    SINK.store(0, std::sync::atomic::Ordering::SeqCst);
    copyt(7);
    if SINK.load(std::sync::atomic::Ordering::SeqCst) != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}