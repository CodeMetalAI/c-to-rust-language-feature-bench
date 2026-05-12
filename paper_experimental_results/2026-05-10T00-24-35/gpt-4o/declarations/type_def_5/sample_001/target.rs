// type_def_5.rs

static mut SINK: i32 = 0;

fn copyt(n: usize) {
    let mut n = n;
    n += 1;

    let mut a: Vec<i32> = Vec::with_capacity(n - 1);
    let mut b: Vec<i32> = vec![0; n];

    let mut i = 0;
    while i < n {
        b[i] = 1000 + i as i32;
        i += 1;
    }

    i = 1;
    while i < n {
        a.push(b[i]);
        i += 1;
    }

    if a.len() as i32 != (n - 1) as i32 {
        unsafe {
            SINK = 1;
        }
    }
    if b.len() as i32 != n as i32 {
        unsafe {
            SINK = 2;
        }
    }

    if a[0] != 1001 {
        unsafe {
            SINK = 3;
        }
    }
    if a[n - 2] != 1000 + (n - 1) as i32 {
        unsafe {
            SINK = 4;
        }
    }
}

fn main() {
    unsafe {
        SINK = 0;
    }
    copyt(7);
    unsafe {
        if SINK != 0 {
            std::process::exit(1);
        }
    }
}