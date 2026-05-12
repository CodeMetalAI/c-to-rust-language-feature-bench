static mut SINK: i32 = 0;

fn copyt(n: usize) {
    let n = n + 1;

    let mut a: Vec<i32> = vec![0; n - 1]; // equivalent to B a
    let mut b: Vec<i32> = vec![0; n]; // equivalent to int b[n]

    for i in 0..n {
        b[i] = 1000 + i as i32;
    }

    for i in 1..n {
        a[i - 1] = b[i];
    }

    if a.len() != n - 1 {
        unsafe { SINK = 1 };
    }
    if b.len() != n {
        unsafe { SINK = 2 };
    }

    if a[0] != 1001 {
        unsafe { SINK = 3 };
    }
    if a[n - 2] != 1000 + (n - 1) as i32 {
        unsafe { SINK = 4 };
    }
}

fn main() {
    unsafe { SINK = 0 };
    copyt(7);
    if unsafe { SINK } != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}