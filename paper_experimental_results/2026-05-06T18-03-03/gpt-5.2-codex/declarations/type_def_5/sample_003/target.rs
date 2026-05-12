use std::sync::atomic::{AtomicI32, Ordering};

static SINK: AtomicI32 = AtomicI32::new(0);

fn copyt(mut n: usize) {
    let orig_n = n;
    n += 1;

    let mut a = vec![0i32; orig_n];
    let mut b = vec![0i32; n];

    let mut i = 0usize;
    while i < n {
        b[i] = 1000 + i as i32;
        i += 1;
    }

    i = 1;
    while i < n {
        a[i - 1] = b[i];
        i += 1;
    }

    if a.len() != n - 1 {
        SINK.store(1, Ordering::SeqCst);
    }
    if b.len() != n {
        SINK.store(2, Ordering::SeqCst);
    }

    if a[0] != 1001 {
        SINK.store(3, Ordering::SeqCst);
    }
    if a[n - 2] != 1000 + (n - 1) as i32 {
        SINK.store(4, Ordering::SeqCst);
    }
}

fn main() {
    SINK.store(0, Ordering::SeqCst);
    copyt(7);
    if SINK.load(Ordering::SeqCst) != 0 {
        std::process::exit(1);
    }
}