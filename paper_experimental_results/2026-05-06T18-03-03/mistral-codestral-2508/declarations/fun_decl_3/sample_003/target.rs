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

fn fpfi<F>(pf: F, k: i32) -> (i32, fn(i32, i32) -> i32)
where
    F: Fn(i64) -> i32,
{
    let t = k as i64 * 97 + 1234;
    let g_seed = pf(t) + k;
    if (g_seed & 1) == 0 {
        (g_seed, v0)
    } else {
        (g_seed, v1)
    }
}

fn call_through<F>(pf: F, g_seed: i32, x: i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    let r1 = pf(g_seed, x);
    let r2 = pf(g_seed, x);
    let r3 = pf(g_seed, x);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let (g_seed, vf) = fpfi(cb0, 17);

    {
        let expected_seed = cb0(17 * 97 + 1234) + 17;
        let base = expected_seed + 40;

        if vf == v0 {
            if vf(g_seed, 40) != base + 1000 {
                std::process::exit(1);
            }
            if vf(g_seed, 40) != base + 1000 {
                std::process::exit(2);
            }
            if call_through(vf, g_seed, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                std::process::exit(3);
            }
        } else {
            if vf(g_seed, 40) != base - 2000 {
                std::process::exit(4);
            }
            if vf(g_seed, 40) != base - 2000 {
                std::process::exit(5);
            }
            if call_through(vf, g_seed, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                std::process::exit(6);
            }
        }
    }

    let (g_seed, vf) = fpfi(cb1, 8);

    {
        let expected_seed = cb1(8 * 97 + 1234) + 8;
        let base = expected_seed + 9;

        if vf == v0 {
            if vf(g_seed, 9) != base + 1000 {
                std::process::exit(7);
            }
            if vf(g_seed, 9) != base + 1000 {
                std::process::exit(8);
            }
            if call_through(vf, g_seed, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                std::process::exit(9);
            }
        } else {
            if vf(g_seed, 9) != base - 2000 {
                std::process::exit(10);
            }
            if vf(g_seed, 9) != base - 2000 {
                std::process::exit(11);
            }
            if call_through(vf, g_seed, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                std::process::exit(12);
            }
        }
    }
}