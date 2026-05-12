use std::process;

static mut SINK: u64 = 0;

fn mix_u64(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
    x ^= x >> 33;
    x
}

fn hash_matrix(n: usize, k: usize, a: &Vec<Vec<f64>>) -> u64 {
    let mut h: u64 = 0x9e3779b97f4a7c15;
    for i in 0..n {
        for j in 0..k {
            let u = a[i][j].to_bits();
            h = mix_u64(h ^ (u.wrapping_add((i * 131 + j * 17) as u64)));
        }
    }
    h
}

fn addscalar(n: usize, m: usize, a: &mut Vec<Vec<f64>>, x: f64) {
    let k = n * m + 300;
    for i in 0..n {
        for j in 0..k {
            a[i][j] += x;
        }
    }
}

fn main() {
    let n: usize = 4;
    let m: usize = 2;
    let k: usize = n * m + 300;
    let mut b: Vec<Vec<f64>> = vec![vec![0.0; k]; n];

    for i in 0..n {
        for j in 0..k {
            b[i][j] = (i * 1000 + j) as f64 * 0.25;
        }
    }

    {
        let before = hash_matrix(n, k, &b);
        addscalar(n, m, &mut b, 2.17);
        let after = hash_matrix(n, k, &b);
        if before == after {
            process::exit(1);
        }
    }

    {
        let x = 2.17;
        for i2 in 0..n {
            for j2 in 0..k {
                let expect = (i2 * 1000 + j2) as f64 * 0.25 + x;
                let got = b[i2][j2];
                let mut diff = got - expect;
                if diff < 0.0 {
                    diff = -diff;
                }
                if diff > 0.0000001 {
                    process::exit(2);
                }
            }
        }
    }

    unsafe {
        SINK ^= b[0][0] as u64;
        if SINK == 0 {
            process::exit(3);
        }
    }

    process::exit(0);
}