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

    // Check conditions
    if a.len() != n - 1 {
        panic!("Size of array `a` is not as expected.");
    }
    if b.len() != n {
        panic!("Size of array `b` is not as expected.");
    }

    if a[0] != 1001 {
        panic!("First element of array `a` is not as expected.");
    }
    if a[n - 2] != 1000 + (n - 1) {
        panic!("Last element of array `a` is not as expected.");
    }
}

fn main() {
    copyt(7);
}