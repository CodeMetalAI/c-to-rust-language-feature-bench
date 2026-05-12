fn copyt(n: usize) {
    let mut a: Vec<i32> = vec![0; n - 1];
    let mut b: Vec<i32> = vec![0; n];

    for i in 0..n {
        b[i] = 1000 + i as i32;
    }

    for i in 1..n {
        a[i - 1] = b[i];
    }

    if a.len() != n - 1 {
        panic!("a length mismatch");
    }
    if b.len() != n {
        panic!("b length mismatch");
    }

    if a[0] != 1001 {
        panic!("a[0] value mismatch");
    }
    if a[n - 2] != 1000 + (n as i32 - 1) {
        panic!("a[n-2] value mismatch");
    }
}

fn main() {
    copyt(7);
}