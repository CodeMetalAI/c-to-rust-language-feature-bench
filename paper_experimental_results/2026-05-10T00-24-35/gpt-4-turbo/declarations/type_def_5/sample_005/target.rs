fn copyt(mut n: usize) {
    n += 1;

    let mut a = vec![0; n - 1];
    let mut b = vec![0; n];

    let mut i = 0;
    while i < n {
        b[i] = 1000 + i;
        i += 1;
    }

    i = 1;
    while i < n {
        a[i - 1] = b[i];
        i += 1;
    }

    assert_eq!(a.len(), n - 1);
    assert_eq!(b.len(), n);

    assert_eq!(a[0], 1001);
    assert_eq!(a[n - 2], 1000 + n - 1);
}

fn main() {
    copyt(7); // In Rust, there's no equivalent to the "sink" mechanism for errors in the original, but we use asserts instead.
}