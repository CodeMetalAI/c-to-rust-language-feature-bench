fn main() {
    let n = 4;
    let m = 2;
    let k = n * m + 300;
    let mut b = [[0.0; 308]; 4];

    for i in 0..n {
        for j in 0..k {
            b[i][j] = (i * 1000 + j) as f64 * 0.25;
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
            let expect = (i * 1000 + j) as f64 * 0.25 + x;
            let got = b[i][j];
            let diff = (got - expect).abs();
            if diff > 0.0000001 {
                std::process::exit(2);
            }
        }
    }

    SINK.with(|sink| {
        let mut sink_val = sink.borrow_mut();
        *sink_val ^= b[0][0].to_bits();
        if *sink_val == 0 {
            std::process::exit(3);
        }
    });

    std::process::exit(0);
}

fn mix_u64(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
    x ^= x >> 33;
    x
}

fn hash_matrix(n: usize, k: usize, a: &[[f64; 308]]) -> u64 {
    let mut h = 0x9e3779b97f4a7c15;
    for i in 0..n {
        for j in 0..k {
            let u = a[i][j].to_bits();
            h = mix_u64(h ^ (u + (i * 131 + j * 17) as u64));
        }
    }
    h
}

fn addscalar(n: usize, m: usize, a: &mut [[f64; 308]], x: f64) {
    let k = n * m + 300;
    for i in 0..n {
        for j in 0..k {
            a[i][j] += x;
        }
    }
}

use std::cell::RefCell;
thread_local! {
    static SINK: RefCell<u64> = RefCell::new(0);
}