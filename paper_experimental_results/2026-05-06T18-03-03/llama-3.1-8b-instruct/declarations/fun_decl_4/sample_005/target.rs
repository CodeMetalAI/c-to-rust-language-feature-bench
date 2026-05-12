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
            let u: u64 = unsafe { std::mem::transmute(a[i][j]) };
            h = mix_u64(h ^ (u + (i as u64 * 131 + j as u64 * 17)));
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
    let k = n * m + 300;

    let mut b: [[f64; k]; n] = [[0.0; k]; n];

    for i in 0..n {
        for j in 0..k {
            b[i][j] = (i as f64 * 1000.0 + j as f64) * 0.25;
        }
    }

    let before = hash_matrix(n, k, &b);

    addscalar(n, m, &mut b, 2.17);

    let after = hash_matrix(n, k, &b);

    if before == after {
        std::process::exit(1);
    }

    let x = 2.17;

    for i in 0..n {
        for j in 0..k {
            let expect = (i as f64 * 1000.0 + j as f64) * 0.25 + x;
            let got = b[i][j];
            let diff = got - expect;
            if diff < 0.0 {
                diff = -diff;
            }
            if diff > 0.0000001 {
                std::process::exit(2);
            }
        }
    }

    let mut sink: u64 = 0;
    sink ^= b[0][0] as u64;
    if sink == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}