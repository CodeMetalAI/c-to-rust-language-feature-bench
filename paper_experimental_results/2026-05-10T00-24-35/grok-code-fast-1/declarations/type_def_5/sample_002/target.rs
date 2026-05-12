fn copyt(n: i32, sink: &mut i32) {
    let n = n + 1;
    let mut b: Vec<i32> = (0..n).map(|i| 1000 + i).collect();
    let mut a: Vec<i32> = Vec::with_capacity((n - 1) as usize);
    for i in 1..n {
        a.push(b[i as usize]);
    }
    if (a.len() as i32) != (n - 1) {
        *sink = 1;
    }
    if (b.len() as i32) != n {
        *sink = 2;
    }
    if a[0] != 1001 {
        *sink = 3;
    }
    if a[((n - 2) as usize)] != 1000 + (n - 1) {
        *sink = 4;
    }
}

fn main() {
    let mut sink = 0;
    copyt(7, &mut sink);
    if sink != 0 {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}