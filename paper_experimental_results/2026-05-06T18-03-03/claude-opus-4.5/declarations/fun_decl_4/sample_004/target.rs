use std::sync::atomic::{AtomicU64, Ordering};

static SINK: AtomicU64 = AtomicU64::new(0);

fn mix_u64(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd_u64);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ceb9fe1a85ec53_u64);
    x ^= x >> 33;
    x
}

fn hash_matrix(n: usize, k: usize, a: &[Vec<f64>]) -> u64 {
    let mut h: u64 = 0x9e3779b97f4a7c15_u64;
    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < k {
            let u: u64 = a[i][j].to_bits();
            h = mix_u64(h ^ (u.wrapping_add((i * 131 + j * 17) as u64)));
            j += 1;
        }
        i += 1;
    }
    h
}

fn addscalar(n: usize, m: usize, a: &mut [Vec<f64>], x: f64) {
    let k = n * m + 300;
    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < k {
            a[i][j] += x;
            j += 1;
        }
        i += 1;
    }
}

fn main() {
    let n: usize = 4;
    let m: usize = 2;

    let k = n * m + 300;

    let mut b: Vec<Vec<f64>> = vec![vec![0.0; 308]; 4];

    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < k {
            b[i][j] = (i * 1000 + j) as f64 * 0.25;
            j += 1;
        }
        i += 1;
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

        let mut i2 = 0;
        while i2 < n {
            let mut j2 = 0;
            while j2 < k {
                let expect = (i2 * 1000 + j2) as f64 * 0.25 + x;
                let got = b[i2][j2];
                let mut diff = got - expect;
                if diff < 0.0 {
                    diff = -diff;
                }
                if diff > 0.0000001 {
                    std::process::exit(2);
                }
                j2 += 1;
            }
            i2 += 1;
        }
    }

    let old_sink = SINK.load(Ordering::SeqCst);
    SINK.store(old_sink ^ (b[0][0].to_bits()), Ordering::SeqCst);
    if SINK.load(Ordering::SeqCst) == 0_u64 {
        std::process::exit(3);
    }

    std::process::exit(0);
}