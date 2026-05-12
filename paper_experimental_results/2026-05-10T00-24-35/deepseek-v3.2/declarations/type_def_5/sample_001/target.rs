fn main() {
    let mut sink = 0;

    copyt(7, &mut sink);

    if sink != 0 {
        std::process::exit(1);
    }
}

fn copyt(n: i32, sink: &mut i32) {
    let n = n + 1;

    let mut a: Vec<i32> = vec![0; n as usize - 1];
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

    if a.len() != (n - 1) as usize {
        *sink = 1;
    }
    if b.len() != n as usize {
        *sink = 2;
    }

    if a[0] != 1001 {
        *sink = 3;
    }
    if a[(n - 2) as usize] != 1000 + (n - 1) {
        *sink = 4;
    }
}