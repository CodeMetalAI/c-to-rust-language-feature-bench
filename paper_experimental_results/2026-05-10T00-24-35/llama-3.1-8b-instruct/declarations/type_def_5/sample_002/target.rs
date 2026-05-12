static mut SINK: i32 = 0;

fn copyt(n: i32) {
    let mut a = Vec::<i32>::with_capacity((n + 1) as usize);
    let mut b = Vec::<i32>::with_capacity(n as usize);

    for i in 0..n {
        b.push(1000 + i);
    }

    for i in 1..n {
        a.push(b[i]);
    }

    if a.len() != (n - 1) as usize {
        unsafe { SINK = 1; }
    }
    if b.len() != n as usize {
        unsafe { SINK = 2; }
    }

    if a[0] != 1001 {
        unsafe { SINK = 3; }
    }
    if a[(n - 2) as usize] != (1000 + (n - 1)) {
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