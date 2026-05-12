fn main() {
    let mut bank_a = [0; 8];
    let mut bank_b = [0; 8];
    let mut c = [0; 8];

    let (a, b) = if true {
        (&mut bank_a, &mut bank_b)
    } else {
        (&mut bank_b, &mut bank_a)
    };

    for i in 0..8 {
        a[i] = 100 + i;
        b[i] = 200 + i;
        c[i] = 300 + i;
    }

    for i in 0..8 {
        a[i] += 1;
        b[i] += 2;
        c[i] += 3;
    }

    let sum_a = a.iter().sum::<i32>();
    let sum_b = b.iter().sum::<i32>();
    let sum_c = c.iter().sum::<i32>();

    if sum_a!= (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
        std::process::exit(1);
    }

    if sum_b!= (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
        std::process::exit(2);
    }

    if sum_c!= (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
        std::process::exit(3);
    }

    if bank_a[0]!= 101 {
        std::process::exit(4);
    }

    if bank_b[7]!= 209 {
        std::process::exit(5);
    }

    if c[0]!= 303 || c[7]!= 310 {
        std::process::exit(6);
    }
}