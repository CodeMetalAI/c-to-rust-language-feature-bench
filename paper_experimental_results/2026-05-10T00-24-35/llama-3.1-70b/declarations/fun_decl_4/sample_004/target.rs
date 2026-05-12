fn mix_u64(x: u64) -> u64 {
    x ^ (x >> 33) * 0xff51afd7ed558ccd * (x ^ (x >> 33) * 0xc4ceb9fe1a85ec53 ^ (x >> 33))
}

fn hash_matrix(a: &[&[f64]]) -> u64 {
    let mut h = 0x9e3779b97f4a7c15;
    for (i, row) in a.iter().enumerate() {
        for (j, &d) in row.iter().enumerate() {
            let u = d.to_bits();
            h = mix_u64(h ^ (u as u64 + (i as u64 * 131 + j as u64 * 17)));
        }
    }
    h
}

fn addscalar(a: &mut [&mut [f64]], x: f64) {
    for row in a {
        for d in row {
            *d += x;
        }
    }
}

fn main() {
    let n = 4;
    let m = 2;
    let k = n * m + 300;

    let mut b: Vec<Vec<f64>> = vec![vec![0.0; k]; n];
    for (i, row) in b.iter_mut().enumerate() {
        for (j, d) in row.iter_mut().enumerate() {
            *d = (i as f64 * 1000.0 + j as f64) * 0.25;
        }
    }

    let before = hash_matrix(&b.iter().map(|x| x.as_slice()).collect::<Vec<_>>());
    addscalar(&mut b.iter_mut().map(|x| x.as_mut_slice()).collect::<Vec<_>>(), 2.17);
    let after = hash_matrix(&b.iter().map(|x| x.as_slice()).collect::<Vec<_>>());
    if before == after {
        std::process::exit(1);
    }

    let x = 2.17;
    for (i, row) in b.iter().enumerate() {
        for (j, &got) in row.iter().enumerate() {
            let expect = (i as f64 * 1000.0 + j as f64) * 0.25 + x;
            let diff = (expect - got).abs();
            if diff > 0.0000001 {
                std::process::exit(2);
            }
        }
    }

    let mut sink = 0u64;
    sink ^= b[0][0].to_bits() as u64;
    if sink == 0 {
        std::process::exit(3);
    }
}