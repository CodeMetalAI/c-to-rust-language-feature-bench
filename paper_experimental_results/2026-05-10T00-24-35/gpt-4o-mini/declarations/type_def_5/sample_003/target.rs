static mut SINK: i32 = 0;

fn copyt(n: usize) {
    let mut n = n + 1;

    let mut a = vec![0; n - 1];
    let mut b = vec![0; n];

    let mut i = 0;
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
        unsafe { SINK = 1; }
    }
    if b.len() != n {
        unsafe { SINK = 2; }
    }

    if a[0] != 1001 {
        unsafe { SINK = 3; }
    }
    if a[n - 2] != 1000 + (n - 1) as i32 {
        unsafe { SINK = 4; }
    }
}

fn main() {
    unsafe { SINK = 0; }
    copyt(7);
    if unsafe { SINK } != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}