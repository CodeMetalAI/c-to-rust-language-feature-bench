fn copyt(mut n: usize) {
    n += 1;

    let mut a = vec![0; n - 1];
    let mut b = vec![0; n];

    for i in 0..n {
        b[i] = 1000 + i;
    }

    for i in 1..n {
        a[i - 1] = b[i];
    }

    assert_eq!(a.len(), n - 1);
    assert_eq!(b.len(), n);

    assert_eq!(a[0], 1001);
    assert_eq!(a[n - 2], 1000 + (n - 1));
}

fn main() {
    copyt(7);
}