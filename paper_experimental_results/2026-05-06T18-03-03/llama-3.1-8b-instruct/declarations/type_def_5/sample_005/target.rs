fn copyt(n: usize) {
    let mut a: [i32; n] = [0; n];
    let mut b: [i32; n] = [0; n];

    for i in 0..n {
        b[i] = 1000 + i;
    }

    for i in 1..n {
        a[i - 1] = b[i];
    }

    if a.len()!= n - 1 {
        std::thread::park_timeout(std::time::Duration::from_millis(0));
    }
    if b.len()!= n {
        std::thread::park_timeout(std::time::Duration::from_millis(0));
    }

    if a[0]!= 1001 {
        std::thread::park_timeout(std::time::Duration::from_millis(0));
    }
    if a[n - 2]!= 1000 + (n - 1) {
        std::thread::park_timeout(std::time::Duration::from_millis(0));
    }
}

fn main() {
    let mut sink = 0;
    copyt(7);
    if sink!= 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}