fn copyt(mut n: usize) {
    n += 1;

    let mut a = vec![0; n - 1];
    let mut b = vec![0; n];

    let mut i = 0;
    while i < n {
        b[i] = 1000 + i;
        i += 1;
    }

    i = 1;
    while i < n {
        a[i - 1] = b[i];
        i += 1;
    }

    // Rust does not need to check the size of arrays as it is guaranteed by the type system
    // and the way we constructed `a` and `b`.

    if a[0] != 1001 {
        unsafe { SINK = 3; }
    }
    if a[n - 2] != 1000 + (n - 1) {
        unsafe { SINK = 4; }
    }
}

static mut SINK: i32 = 0;

fn main() {
    unsafe {
        SINK = 0;
        copyt(7);
        if SINK != 0 {
            std::process::exit(1);
        }
    }
}