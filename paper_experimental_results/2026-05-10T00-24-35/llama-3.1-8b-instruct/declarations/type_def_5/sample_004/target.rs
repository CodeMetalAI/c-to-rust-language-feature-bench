fn copyt(n: usize) {
    let mut a: [i32; n + 1] = [0; n + 1];
    let mut b: [i32; n + 1] = [0; n + 1];

    for i in 1..=n {
        b[i] = 1000 + i;
    }

    for i in 1..n {
        a[i - 1] = b[i];
    }

    if a.len()!= n {
        std::sync::atomic::AtomicI32::new(0).store(1, std::sync::atomic::Ordering::SeqCst);
    }
    if b.len()!= n + 1 {
        std::sync::atomic::AtomicI32::new(0).store(2, std::sync::atomic::Ordering::SeqCst);
    }

    if a[0]!= 1001 {
        std::sync::atomic::AtomicI32::new(0).store(3, std::sync::atomic::Ordering::SeqCst);
    }
    if a[n - 2]!= 1000 + (n - 1) {
        std::sync::atomic::AtomicI32::new(0).store(4, std::sync::atomic::Ordering::SeqCst);
    }
}

fn main() {
    let sink = std::sync::atomic::AtomicI32::new(0);
    copyt(7);
    if sink.load(std::sync::atomic::Ordering::SeqCst)!= 0 {
        return 1;
    }
    return 0;
}