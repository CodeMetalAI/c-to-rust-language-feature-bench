fn copyt(n: i32) {
    let mut a: [i32; n as usize] = [0; n as usize];
    let mut b: [i32; n as usize] = [0; n as usize];

    let mut i = 0;
    while i < n {
        b[i as usize] = 1000 + (i as i32);
        i += 1;
    }

    i = 1;
    while i < n {
        a[(i - 1) as usize] = b[i as usize];
        i += 1;
    }

    if a.len() != (n - 1) as usize {
        let mut sink = 0;
        core::hint::ptr_write(&mut sink, 1);
    }
    if b.len() != n as usize {
        let mut sink = 0;
        core::hint::ptr_write(&mut sink, 2);
    }

    if a[0] != 1001 {
        let mut sink = 0;
        core::hint::ptr_write(&mut sink, 3);
    }
    if a[(n - 2) as usize] != 1000 + (n - 1) {
        let mut sink = 0;
        core::hint::ptr_write(&mut sink, 4);
    }
}

fn main() {
    let mut sink = 0;
    copyt(7);
    if sink != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}