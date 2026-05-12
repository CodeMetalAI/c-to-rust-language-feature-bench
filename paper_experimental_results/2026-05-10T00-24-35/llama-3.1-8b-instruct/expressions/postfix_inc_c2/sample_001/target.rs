use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    let x = AtomicI32::new(41);

    let old = x.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if old!= 41 {
        return 1;
    }
    if x.load(std::sync::atomic::Ordering::Relaxed)!= 42 {
        return 2;
    }

    old = x.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if old!= 42 {
        return 3;
    }
    if x.load(std::sync::atomic::Ordering::Relaxed)!= 43 {
        return 4;
    }

    x.store(100, std::sync::atomic::Ordering::Relaxed);
    let y = x.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 0;
    if y!= 100 {
        return 5;
    }
    if x.load(std::sync::atomic::Ordering::Relaxed)!= 101 {
        return 6;
    }

    0
}