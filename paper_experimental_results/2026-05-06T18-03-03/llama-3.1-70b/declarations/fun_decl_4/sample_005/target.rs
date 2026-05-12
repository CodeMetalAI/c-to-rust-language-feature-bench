fn mix_u64(x: u64) -> u64 {
    x ^ (x >> 33) * 0xff51afd7ed558ccd ^ (x >> 33) * 0xc4ceb9fe1a85ec53 ^ (x >> 33)
}

fn hash_matrix(n: usize, k: usize, a: &[[f64; 308]]) -> u64 {
    let mut h = 0x9e3779b97f4a7c15;
    for i in 0..n {
        for j in 0..k {
            let bytes = a[i][j].to_le_bytes();
            let u = u64::from_le_bytes(bytes);
            h = mix_u64(h ^ (u + (i as u64 * 131 + j as u64 * 17)));
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

fn main() {
    let n = 4;
    let m = 2;
    let k = n * m + 300;

    let mut b = [[0.0; 308]; 4];

    for i in 0..n {
        for j in 0..k {
            b[i][j] = (i as f64 * 1000.0 + j as f64) * 0.25;
        }
    }

    {
        let before = hash_matrix(n, &b);
        addscalar(n, m, &mut b, 2.17);
        let after = hash_matrix(n, &b);
        if before == after {
            std::process::exit(1);
        }
    }

    {
        let x = 2.17;
        for i in 0..n {
            for j in 0..k {
                let expect = (i as f64 * 1000.0 + j as f64) * 0.25 + x;
                let got = b[i][j];
                let diff = (got - expect).abs();
                if diff > 0.0000001 {
                    std::process::exit(2);
                }
            }
        }
    }

    let mut sink = 0;
    sink ^= b[0][0] as u64;
    if sink == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}