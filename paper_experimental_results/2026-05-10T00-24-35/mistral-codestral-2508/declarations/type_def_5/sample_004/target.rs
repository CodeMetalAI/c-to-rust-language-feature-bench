fn copyt(n: usize) {
    let mut a: Vec<i32> = Vec::with_capacity(n);
    let mut b: Vec<i32> = Vec::with_capacity(n + 1);

    for i in 0..n + 1 {
        b.push(1000 + i as i32);
    }

    for i in 1..n + 1 {
        a.push(b[i]);
    }

    if a.len() != n {
        eprintln!("Error: a.len() != n");
        std::process::exit(1);
    }
    if b.len() != n + 1 {
        eprintln!("Error: b.len() != n + 1");
        std::process::exit(2);
    }

    if a[0] != 1001 {
        eprintln!("Error: a[0] != 1001");
        std::process::exit(3);
    }
    if a[n - 1] != 1000 + (n as i32) {
        eprintln!("Error: a[n - 1] != 1000 + n");
        std::process::exit(4);
    }
}

fn main() {
    copyt(7);
}