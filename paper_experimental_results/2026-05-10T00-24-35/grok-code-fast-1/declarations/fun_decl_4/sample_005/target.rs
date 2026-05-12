fn mix_u64(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
    x ^= x >> 33;
    x
}

fn hash_matrix(n: i32, k: i32, a: &Vec<Vec<f64>>) -> u64 {
    let mut h = 0x9e3779b97f4a7c15u64;
    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < k {
            let d = a[i as usize][j as usize];
            let u = d.to_bits();
            h = mix_u64(h ^ (u.wrapping_add((i as u64 * 131 + j as u64 * 17) as u64)));
            j += 1;
        }
        i += 1;
    }
    h
}

fn addscalar(n: i32, m: i32, a: &mut Vec<Vec<f64>>, x: f64) {
    let k = n * m + 300;
    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < k {
            a[i as usize][j as usize] += x;
            j += 1;
        }
        i += 1;
    }
}

fn main() {
    let n = 4;
    let m = 2;
    let k = n * m + 300;
    let mut b: Vec<Vec<f64>> = vec![vec![0.0; k as usize]; n as usize];
    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < k {
            b[i as usize][j as usize] = (i * 1000 + j) as f64 * 0.25;
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
                let got = b[i2 as usize][j2 as usize];
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
    let mut sink: u64 = 0;
    sink ^= b[0][0] as u64;
    if sink == 0u64 {
        std::process::exit(3);
    }
    std::process::exit(0);
}