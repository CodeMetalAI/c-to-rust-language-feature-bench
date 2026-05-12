fn copyt(n: usize) {
    let mut b: Vec<i32> = vec![0; n];
    let mut a: Vec<i32> = vec![0; n - 1];

    for i in 0..n {
        b[i] = 1000 + i as i32;
    }

    for i in 1..n {
        a[i - 1] = b[i];
    }

    if a.len()!= n - 1 {
        std::process::exit(1);
    }
    if b.len()!= n {
        std::process::exit(2);
    }

    if a[0]!= 1001 {
        std::process::exit(3);
    }
    if a[n - 2]!= (1000 + (n - 1) as i32) {
        std::process::exit(4);
    }
}

fn main() {
    copyt(7);
    std::process::exit(0);
}