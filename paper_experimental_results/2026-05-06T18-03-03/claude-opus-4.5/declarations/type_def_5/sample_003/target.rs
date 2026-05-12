use std::cell::Cell;

thread_local! {
    static SINK: Cell<i32> = Cell::new(0);
}

fn copyt(mut n: i32) {
    let size_a = n as usize;

    n += 1;

    let mut a: Vec<i32> = vec![0; size_a];
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
        SINK.with(|s| s.set(1));
    }
    if b.len() as i32 != n {
        SINK.with(|s| s.set(2));
    }

    if a[0] != 1001 {
        SINK.with(|s| s.set(3));
    }
    if a[(n - 2) as usize] != 1000 + (n - 1) {
        SINK.with(|s| s.set(4));
    }
}

fn main() {
    SINK.with(|s| s.set(0));
    copyt(7);
    if SINK.with(|s| s.get()) != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}