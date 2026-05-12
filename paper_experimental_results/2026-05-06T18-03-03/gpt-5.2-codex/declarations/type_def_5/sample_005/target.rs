use std::sync::atomic::{AtomicI32, Ordering};

static SINK: AtomicI32 = AtomicI32::new(0);

fn copyt(n: i32) {
    let orig_n = n as usize;
    let n_inc = n + 1;
    let n_usize = n_inc as usize;

    let mut a = vec![0i32; orig_n];
    let mut b = vec![0i32; n_usize];

    let mut i = 0usize;
    while i < n_usize {
        b[i] = 1000 + i as i32;
        i += 1;
    }

    i = 1;
    while i < n_usize {
        a[i - 1] = b[i];
        i += 1;
    }

    if a.len() as i32 != n_inc - 1 {
        SINK.store(1, Ordering::SeqCst);
    }
    if b.len() as i32 != n_inc {
        SINK.store(2, Ordering::SeqCst);
    }

    if a[0] != 1001 {
        SINK.store(3, Ordering::SeqCst);
    }
    if a[n_usize - 2] != 1000 + (n_inc - 1) {
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