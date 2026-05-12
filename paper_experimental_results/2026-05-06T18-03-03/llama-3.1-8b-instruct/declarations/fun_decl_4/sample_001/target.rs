use std::f64::consts::PI;

fn mix_u64(x: u64) -> u64 {
    x ^= x >> 33;
    x *= 0xff51afd7ed558ccd;
    x ^= x >> 33;
    x *= 0xc4ceb9fe1a85ec53;
    x ^= x >> 33;
    x
}

fn hash_matrix(n: usize, k: usize, a: &[[f64; k]; n]) -> u64 {
    let mut h = 0x9e3779b97f4a7c15;
    for i in 0..n {
        for j in 0..k {
            let u: u64 = (a[i][j] as u64) + (i as u64 * 131 + j as u64 * 17) as u64;
            h = mix_u64(h ^ u);
        }
    }
    h
}

fn addscalar(n: usize, m: usize, a: &mut [[f64; n * m + 300]; n], x: f64) {
    let k = n * m + 300;
    for i in 0..n {
        for j in 0..k {
            a[i][j] += x;
        }
    }
}

fn main() {
    let n = 4;
    let m = 2;

    let mut b: [[f64; 308]; 4] = [[0.0; 308]; 4];

    for i in 0..n {
        for j in 0..308 {
            b[i][j] = (i * 1000 + j as f64) * 0.25;
        }
    }

    let before = hash_matrix(n, 308, &b);

    addscalar(n, m, &mut b, 2.17);

    let after = hash_matrix(n, 308, &b);

    if before == after {
        return;
    }

    for i in 0..n {
        for j in 0..308 {
            let expect = (i * 1000 + j as f64) * 0.25 + 2.17;
            let got = b[i][j];
            let diff = (got - expect).abs();
            if diff > 0.0000001 {
                return;
            }
        }
    }

    let mut sink = 0;
    sink ^= b[0][0] as u64;
    if sink == 0 {
        return;
    }

    std::process::exit(0);
}