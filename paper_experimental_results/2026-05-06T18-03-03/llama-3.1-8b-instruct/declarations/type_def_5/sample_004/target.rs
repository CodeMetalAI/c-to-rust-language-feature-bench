fn copyt(n: usize) {
    let mut a = vec![0; n];
    let mut b = vec![0; n];

    for i in 1..n {
        b[i] = 1000 + i;
    }

    for i in 1..n {
        a[i - 1] = b[i];
    }

    if a.len()!= n - 1 {
        std::sync::atomic::AtomicI32::new(1).store(1, std::sync::atomic::Ordering::SeqCst);
    }
    if b.len()!= n {
        std::sync::atomic::AtomicI32::new(2).store(2, std::sync::atomic::Ordering::SeqCst);
    }

    if a[0]!= 1001 {
        std::sync::atomic::AtomicI32::new(3).store(3, std::sync::atomic::Ordering::SeqCst);
    }
    if a[n - 2]!= 1000 + (n - 1) {
        std::sync::atomic::AtomicI32::new(4).store(4, std::sync::atomic::Ordering::SeqCst);
    }
}

fn main() {
    let mut sink = std::sync::atomic::AtomicI32::new(0);
    copyt(7);
    if sink.load(std::sync::atomic::Ordering::SeqCst)!= 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}