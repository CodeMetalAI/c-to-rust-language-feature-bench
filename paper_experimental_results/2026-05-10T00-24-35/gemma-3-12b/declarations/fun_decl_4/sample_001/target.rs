fn main() {
    let n = 4;
    let m = 2;

    let k = n * m + 300;

    let mut b: Vec<Vec<f64>> = vec![vec![0.0; k]; n];

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
            return 1;
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
                let diff = if got - expect < 0.0 {
                    -(got - expect)
                } else {
                    got - expect
                };
                if diff > 0.0000001 {
                    return 2;
                }
                j2 += 1;
            }
            i2 += 1;
        }
    }

    let mut sink: u64 = 0;
    sink ^= b[0][0] as u64;
    if sink == 0 {
        return 3;
    }

    0
}

fn mix_u64(x: u64) -> u64 {
    let mut x = x ^ (x >> 33);
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x = x ^ (x >> 33);
    x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
    x = x ^ (x >> 33);
    x
}

fn hash_matrix(n: usize, k: usize, a: &Vec<Vec<f64>>) -> u64 {
    let mut h: u64 = 0x9e3779b97f4a7c15;
    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < k {
            let u = {
                let d = a[i][j];
                u64::from_f64(d).unwrap()
            };
            h = mix_u64(h ^ (u + (i * 131 + j * 17) as u64));
            j += 1;
        }
        i += 1;
    }
    h
}

fn addscalar(n: usize, m: usize, a: &mut Vec<Vec<f64>>, x: f64) {
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