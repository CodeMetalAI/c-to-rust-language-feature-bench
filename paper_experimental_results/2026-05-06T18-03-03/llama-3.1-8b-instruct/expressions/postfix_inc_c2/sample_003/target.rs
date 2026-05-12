use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    let mut x = AtomicI32::new(41);

    let old = x.fetch_add(1, Ordering::SeqCst);
    if old != 41 {
        return;
    }
    if x.load(Ordering::SeqCst) != 42 {
        return;
    }

    old = x.fetch_add(1, Ordering::SeqCst);
    if old != 42 {
        return;
    }
    if x.load(Ordering::SeqCst) != 43 {
        return;
    }

    x.store(100, Ordering::SeqCst);
    let y = x.fetch_add(1, Ordering::SeqCst) + 0;
    if y != 100 {
        return;
    }
    if x.load(Ordering::SeqCst) != 101 {
        return;
    }

    ()
}