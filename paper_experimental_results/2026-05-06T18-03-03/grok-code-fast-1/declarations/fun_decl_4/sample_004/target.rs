static mut SINK: u64 = 0;

fn mix_u64(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
    x ^= x >> 33;
    x
}

fn hash_matrix(a: &[[f64; 308]; 4]) -> u64 {
    let mut h: u64 = 0x9e3779b97f4a7c15;
    for i in 0..4 {
        for j in 0..308 {
            let u = a[i][j].to_bits();
            h = mix_u64(h ^ (u.wrapping_add((i as u64).wrapping_mul(131).wrapping_add((j as u64).wrapping_mul(17)))));
        }
    }
    h
}

fn addscalar(a: &mut [[f64; 308]; 4], x: f64) {
    for i in 0..4 {
        for j in 0..308 {
            a[i][j] += x;
        }
    }
}

fn main() {
    let n = 4;
    let m = 2;
    let k = n * m + 300;
    let mut b: [[f64; 308]; 4] = [[0.0; 308]; 4];
    for i in 0..n {
        for j in 0..k {
            b[i][j] = (i as f64 * 1000.0 + j as f64) * 0.25;
        }
    }
    {
        let before = hash_matrix(&b);
        addscalar(&mut b, 2.17);
        let after = hash_matrix(&b);
        if before == after {
            std::process::exit(1);
        }
    }
    {
        let x = 2.17;
        for i2 in 0..n {
            for j2 in 0..k {
                let expect = (i2 as f64 * 1000.0 + j2 as f64) * 0.25 + x;
                let got = b[i2][j2];
                let diff = (got - expect).abs();
                if diff > 0.0000001 {
                    std::process::exit(2);
                }
            }
        }
    }
    unsafe {
        SINK ^= b[0][0].to_bits();
        if SINK == 0 {
            std::process::exit(3);
        }
    }
    std::process::exit(0);
}