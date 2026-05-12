fn mix_u64(x: u64) -> u64 {
    x ^= x >> 33;
    x.wrapping_mul(0xff51afd7ed558ccd)
       .wrapping_xor((x >> 33))
       .wrapping_mul(0xc4ceb9fe1a85ec53)
       .wrapping_xor(x >> 33)
}

fn hash_matrix(a: &[[f64; 308]]) -> u64 {
    let mut h = 0x9e3779b97f4a7c15;
    for (i, row) in a.iter().enumerate() {
        for (j, &d) in row.iter().enumerate() {
            let u = d.to_bits() as u64;
            h = mix_u64(h ^ (u + ((i * 131 + j * 17) as u64)));
        }
    }
    h
}

fn add_scalar(a: &mut [[f64; 308]], x: f64) {
    for row in a.iter_mut() {
        for elem in row.iter_mut() {
            *elem += x;
        }
    }
}

fn main() {
    let mut b = [[0.0; 308]; 4];
    for (i, row) in b.iter_mut().enumerate() {
        for (j, elem) in row.iter_mut().enumerate() {
            *elem = ((i * 1000 + j) as f64) * 0.25;
        }
    }

    {
        let before = hash_matrix(&b);
        add_scalar(&mut b, 2.17);
        let after = hash_matrix(&b);
        if before == after {
            std::process::exit(1);
        }
    }

    {
        let x = 2.17;
        for (i, row) in b.iter().enumerate() {
            for (j, &got) in row.iter().enumerate() {
                let expect = ((i * 1000 + j) as f64) * 0.25 + x;
                let diff = (got - expect).abs();
                if diff > 0.0000001 {
                    std::process::exit(2);
                }
            }
        }
    }

    let mut sink = 0;
    sink ^= b[0][0].to_bits() as u64;
    if sink == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}