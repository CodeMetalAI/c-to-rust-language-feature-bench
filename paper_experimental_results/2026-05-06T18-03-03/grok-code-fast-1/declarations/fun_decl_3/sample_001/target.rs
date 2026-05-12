fn cb0(x: i64) -> i32 {
    ((x ^ 0x13579i64) as i32) + 7
}

fn cb1(x: i64) -> i32 {
    ((x * 3i64) as i32) - 11
}

fn fpfi(pf: fn(i64) -> i32, k: i32) -> (Box<dyn Fn(i32) -> i32>, bool) {
    let t = (k as i64) * 97i64 + 1234i64;
    let seed = pf(t) + k;
    let is_even = (seed & 1) == 0;
    let closure = if is_even {
        Box::new(move |x: i32| seed + x + 1000)
    } else {
        Box::new(move |x: i32| seed + x - 2000)
    };
    (closure, is_even)
}

fn call_through(pf: &dyn Fn(i32) -> i32, x: i32) -> i32 {
    let r1 = pf(x);
    let r2 = pf(x);
    let r3 = pf(x);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let (vf, is_v0) = fpfi(cb0, 17);
    {
        let expected_seed = cb0((17i64) * 97i64 + 1234i64) + 17;
        let base = expected_seed + 40;
        if is_v0 {
            if vf(40) != base + 1000 {
                std::process::exit(1);
            }
            if vf(40) != base + 1000 {
                std::process::exit(2);
            }
            if call_through(&vf, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                std::process::exit(3);
            }
        } else {
            if vf(40) != base - 2000 {
                std::process::exit(4);
            }
            if vf(40) != base - 2000 {
                std::process::exit(5);
            }
            if call_through(&vf, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                std::process::exit(6);
            }
        }
    }

    let (vf, is_v0) = fpfi(cb1, 8);
    {
        let expected_seed = cb1((8i64) * 97i64 + 1234i64) + 8;
        let base = expected_seed + 9;
        if is_v0 {
            if vf(9) != base + 1000 {
                std::process::exit(7);
            }
            if vf(9) != base + 1000 {
                std::process::exit(8);
            }
            if call_through(&vf, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                std::process::exit(9);
            }
        } else {
            if vf(9) != base - 2000 {
                std::process::exit(10);
            }
            if vf(9) != base - 2000 {
                std::process::exit(11);
            }
            if call_through(&vf, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                std::process::exit(12);
            }
        }
    }
    std::process::exit(0);
}