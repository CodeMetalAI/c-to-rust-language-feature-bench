fn mix_u64(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
    x ^= x >> 33;
    x
}

fn hash_matrix(n: usize, k: usize, a: &[f64]) -> u64 {
    let mut h: u64 = 0x9e3779b97f4a7c15;
    for i in 0..n {
        for j in 0..k {
            let idx = i * k + j;
            let val = a[idx];
            let u = val.to_bits();
            h = mix_u64(h ^ (u.wrapping_add(i as u64 * 131 + j as u64 * 17)));
        }
    }
    h
}

fn addscalar(n: usize, m: usize, a: &mut [f64], x: f64) {
    let k = n * m + 300;
    for i in 0..n {
        for j in 0..k {
            let idx = i * k + j;
            a[idx] += x;
        }
    }
}

fn main() {
    let n: usize = 4;
    let m: usize = 2;
    let k: usize = n * m + 300;
    let mut b: Vec<f64> = vec![0.0; n * k];
    for i in 0..n {
        for j in 0..k {
            let idx = i * k + j;
            b[idx] = (i * 1000 + j) as f64 * 0.25;
        }
    }
    {
        let before = hash_matrix(n, k, &b);
        addscalar(n, m, &mut b, 2.17);
        let after = hash_matrix(n, k, &b);
        if before == after {
            std::process::exit(1);
        }
    }
    {
        let x = 2.17;
        for i2 in 0..n {
            for j2 in 0..k {
                let idx = i2 * k + j2;
                let expect = (i2 * 1000 + j2) as f64 * 0.25 + x;
                let got = b[idx];
                let mut diff = got - expect;
                if diff < 0.0 {
                    diff = -diff;
                }
                if diff > 1e-7 {
                    std::process::exit(2);
                }
            }
        }
    }
    let mut sink: u64 = 0;
    sink ^= b[0].to_bits();
    if sink == 0 {
        std::process::exit(3);
    }
}