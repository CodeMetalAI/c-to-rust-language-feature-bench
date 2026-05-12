use std::sync::atomic::{AtomicI32, Ordering};
use std::process;

static SINK: AtomicI32 = AtomicI32::new(0);

fn copyt(n_in: i32) {
    let orig_n = n_in as usize;
    let n = (n_in + 1) as usize;

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

    if a.len() as i32 != (n as i32 - 1) {
        SINK.store(1, Ordering::SeqCst);
    }
    if b.len() as i32 != n as i32 {
        SINK.store(2, Ordering::SeqCst);
    }

    if a[0] != 1001 {
        SINK.store(3, Ordering::SeqCst);
    }
    if a[n - 2] != 1000 + (n as i32 - 1) {
        SINK.store(4, Ordering::SeqCst);
    }
}

fn main() {
    SINK.store(0, Ordering::SeqCst);
    copyt(7);
    if SINK.load(Ordering::SeqCst) != 0 {
        process::exit(1);
    }
    process::exit(0);
}