fn cb0(x: i64) -> i32 {
    (x ^ 0x13579) as i32 + 7
}

fn cb1(x: i64) -> i32 {
    (x * 3) as i32 - 11
}

fn v0(g_seed: i32, x: i32) -> i32 {
    g_seed + x + 1000
}

fn v1(g_seed: i32, x: i32) -> i32 {
    g_seed + x - 2000
}

fn fpfi(pf: fn(i64) -> i32, k: i32) -> fn(i32, i32) -> i32 {
    let t = k as i64 * 97 + 1234;
    let g_seed = pf(t) + k;
    if (g_seed & 1) == 0 {
        move |x: i32, _: i32| v0(g_seed, x)
    } else {
        move |x: i32, _: i32| v1(g_seed, x)
    }
}

fn call_through(vf: fn(i32, i32) -> i32, x: i32) -> i32 {
    let r1 = vf(x, 0);
    let r2 = vf(x, 1);
    let r3 = vf(x, 2);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0(17 * 97 + 1234) + 17;
        let base = expected_seed + 40;

        if vf == v0 {
            if vf(40, 0) != base + 1000 {
                std::process::exit(1);
            }
            if vf(40, 1) != base + 1000 {
                std::process::exit(2);
            }
            if call_through(vf, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                std::process::exit(3);
            }
        } else {
            if vf(40, 0) != base - 2000 {
                std::process::exit(4);
            }
            if vf(40, 1) != base - 2000 {
                std::process::exit(5);
            }
            if call_through(vf, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                std::process::exit(6);
            }
        }
    }

    let vf = fpfi(cb1, 8);

    {
        let expected_seed = cb1(8 * 97 + 1234) + 8;
        let base = expected_seed + 9;

        if vf == v0 {
            if vf(9, 0) != base + 1000 {
                std::process::exit(7);
            }
            if vf(9, 1) != base + 1000 {
                std::process::exit(8);
            }
            if call_through(vf, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                std::process::exit(9);
            }
        } else {
            if vf(9, 0) != base - 2000 {
                std::process::exit(10);
            }
            if vf(9, 1) != base - 2000 {
                std::process::exit(11);
            }
            if call_through(vf, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                std::process::exit(12);
            }
        }
    }
}