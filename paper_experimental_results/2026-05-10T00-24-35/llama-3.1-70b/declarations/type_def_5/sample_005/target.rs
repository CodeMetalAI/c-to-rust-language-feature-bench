fn copyt(n: usize) {
    let mut b = vec![0; n];
    for i in 0..n {
        b[i] = 1000 + i;
    }

    let mut a = Vec::with_capacity(n - 1);
    a.extend_from_slice(&b[1..]);

    if a.len()!= n - 1 {
        std::process::exit(1);
    }
    if b.len()!= n {
        std::process::exit(2);
    }

    if a[0]!= 1001 {
        std::process::exit(3);
    }
    if a[n - 2]!= 1000 + (n - 1) {
        std::process::exit(4);
    }
}

fn main() {
    copyt(7);
}