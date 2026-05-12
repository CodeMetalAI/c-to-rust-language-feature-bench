fn copyt(n: usize) {
    let mut a: Vec<i32> = vec![0; n];
    let mut b: Vec<i32> = vec![0; n + 1];

    for i in 0..n + 1 {
        b[i] = 1000 + i as i32;
    }

    for i in 1..n + 1 {
        a[i - 1] = b[i];
    }

    if a.len() != n {
        panic!("a length mismatch");
    }
    if b.len() != n + 1 {
        panic!("b length mismatch");
    }

    if a[0] != 1001 {
        panic!("a[0] value mismatch");
    }
    if a[n - 1] != 1000 + (n as i32) {
        panic!("a[n-1] value mismatch");
    }
}

fn main() {
    copyt(7);
}