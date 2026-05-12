use std::process::exit;
use std::sync::atomic::{AtomicU64, Ordering};

static SINK: AtomicU64 = AtomicU64::new(0);

fn mix_u64(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
    x ^= x >> 33;
    x
}

fn hash_matrix(n: usize, k: usize, a: &[[f64; 308]; 4]) -> u64 {
    let mut h: u64 = 0x9e3779b97f4a7c15;
    let mut i = 0usize;
    while i < n {
        let mut j = 0usize;
        while j < k {
            let u = a[i][j].to_bits();
            let add = (i * 131 + j * 17) as u64;
            h = mix_u64(h ^ u.wrapping_add(add));
            j += 1;
        }
        i += 1;
    }
    h
}

fn addscalar(n: usize, m: usize, a: &mut [[f64; 308]; 4], x: f64) {
    let k = n * m + 300;
    let mut i = 0usize;
    while i < n {
        let mut j = 0usize;
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

    let mut b = [[0f64; 308]; 4];

    let mut i = 0usize;
    while i < n {
        let mut j = 0usize;
        while j < k {
            b[i][j] = (i as f64 * 1000.0 + j as f64) * 0.25;
            j += 1;
        }
        i += 1;
    }

    {
        let before = hash_matrix(n, k, &b);
        addscalar(n, m, &mut b, 2.17);
        let after = hash_matrix(n, k, &b);
        if before == after {
            exit(1);
        }
    }

    {
        let x = 2.17;
        let mut i2 = 0usize;
        while i2 < n {
            let mut j2 = 0usize;
            while j2 < k {
                let expect = (i2 as f64 * 1000.0 + j2 as f64) * 0.25 + x;
                let got = b[i2][j2];
                let mut diff = got - expect;
                if diff < 0.0 {
                    diff = -diff;
                }
                if diff > 0.0000001 {
                    exit(2);
                }
                j2 += 1;
            }
            i2 += 1;
        }
    }

    let val = b[0][0] as u64;
    let new_val = SINK.fetch_xor(val, Ordering::SeqCst) ^ val;
    if new_val == 0 {
        exit(3);
    }

    exit(0);
}