// type_def_5.rs

static mut SINK: i32 = 0;

fn copyt(n: usize) {
    let mut n = n;
    n += 1;

    let mut a = vec![0; n - 1];
    let mut b = vec![0; n];

    for i in 0..n {
        b[i] = 1000 + i;
    }

    for i in 1..n {
        a[i - 1] = b[i];
    }

    unsafe {
        if a.len() != (n - 1) {
            SINK = 1;
        }
        if b.len() != n {
            SINK = 2;
        }
        if a[0] != 1001 {
            SINK = 3;
        }
        if a[n - 2] != 1000 + (n - 1) {
            SINK = 4;
        }
    }
}

fn main() {
    unsafe {
        SINK = 0;
        copyt(7);
        if SINK != 0 {
            std::process::exit(1);
        }
    }
}