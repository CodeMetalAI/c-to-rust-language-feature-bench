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

fn fpfi<F>(pf: F, k: i32) -> Box<dyn Fn(i32) -> i32>
where
    F: Fn(i64) -> i32,
{
    let t = k as i64 * 97 + 1234;
    let g_seed = pf(t) + k;
    if (g_seed & 1) == 0 {
        Box::new(move |x| v0(g_seed, x))
    } else {
        Box::new(move |x| v1(g_seed, x))
    }
}

fn call_through<F>(pf: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    let r1 = pf(x);
    let r2 = pf(x);
    let r3 = pf(x);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0(17 * 97 + 1234) + 17;
        let base = expected_seed + 40;

        if vf(40) != base + 1000 {
            std::process::exit(1);
        }
        if vf(40) != base + 1000 {
            std::process::exit(2);
        }
        if call_through(&vf, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
            std::process::exit(3);
        }
    }

    let vf = fpfi(cb1, 8);

    {
        let expected_seed = cb1(8 * 97 + 1234) + 8;
        let base = expected_seed + 9;

        if vf(9) != base + 1000 {
            std::process::exit(7);
        }
        if vf(9) != base + 1000 {
            std::process::exit(8);
        }
        if call_through(&vf, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
            std::process::exit(9);
        }
    }
}