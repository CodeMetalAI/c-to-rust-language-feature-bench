fn main() {
    let mut sink = 0;

    let n = 7;
    let mut a: [i32; n] = [0; n];
    let mut b: [i32; n] = [0; n];

    for i in 0..n {
        b[i] = 1000 + i;
    }

    for i in 1..n {
        a[i - 1] = b[i];
    }

    if a.len() != n - 1 {
        sink = 1;
    }
    if b.len() != n {
        sink = 2;
    }

    if a[0] != 1001 {
        sink = 3;
    }
    if a[n - 2] != 1000 + (n - 1) {
        sink = 4;
    }

    if sink != 0 {
        std::process::exit(1);
    }
}