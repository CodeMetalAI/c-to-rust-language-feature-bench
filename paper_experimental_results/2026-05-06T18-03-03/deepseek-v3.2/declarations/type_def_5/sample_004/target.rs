use std::sync::atomic::{AtomicI32, Ordering};

static SINK: AtomicI32 = AtomicI32::new(0);

fn copyt(n: i32) {
    let n = n as usize;
    let mut n_var = n;
    n_var += 1;

    let mut a: Vec<i32> = vec![0; n];
    let mut b: Vec<i32> = vec![0; n_var];

    let mut i = 0;
    while i < n_var {
        b[i] = 1000 + i as i32;
        i += 1;
    }

    i = 1;
    while i < n_var {
        a[i - 1] = b[i];
        i += 1;
    }

    if a.len() != n_var - 1 {
        SINK.store(1, Ordering::SeqCst);
    }
    if b.len() != n_var {
        SINK.store(2, Ordering::SeqCst);
    }

    if a[0] != 1001 {
        SINK.store(3, Ordering::SeqCst);
    }
    if a[n_var - 2] != 1000 + (n_var - 1) as i32 {
        SINK.store(4, Ordering::SeqCst);
    }
}

fn main() {
    SINK.store(0, Ordering::SeqCst);
    copyt(7);
    if SINK.load(Ordering::SeqCst) != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}