use std::sync::atomic::{AtomicI32, Ordering};
use std::process::exit;

static SINK: AtomicI32 = AtomicI32::new(0);

fn copyt(n: i32) {
    let mut n = n + 1;

    let mut a: Vec<i32> = vec![0; (n - 1) as usize];
    let mut b: Vec<i32> = vec![0; n as usize];

    let mut i = 0;
    while i < n {
        b[i as usize] = 1000 + i;
        i += 1;
    }

    i = 1;
    while i < n {
        a[(i - 1) as usize] = b[i as usize];
        i += 1;
    }

    if a.len() as i32 != (n - 1) {
        SINK.store(1, Ordering::SeqCst);
    }
    if b.len() as i32 != n {
        SINK.store(2, Ordering::SeqCst);
    }

    if a[0] != 1001 {
        SINK.store(3, Ordering::SeqCst);
    }
    if a[(n - 2) as usize] != 1000 + (n - 1) {
        SINK.store(4, Ordering::SeqCst);
    }
}

fn main() {
    SINK.store(0, Ordering::SeqCst);
    copyt(7);
    if SINK.load(Ordering::SeqCst) != 0 {
        exit(1);
    } else {
        exit(0);
    }
}