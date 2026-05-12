fn cb0(x: i64) -> i32 {
    (x ^ 0x13579L) as i32 + 7
}

fn cb1(x: i64) -> i32 {
    (x * 3L) as i32 - 11
}

fn v0(x: i32) -> i32 {
    let g_seed = std::thread::local!().wrap(g_seed);
    g_seed.with(|| x + 1000 + g_seed.get())
}

fn v1(x: i32) -> i32 {
    let g_seed = std::thread::local!().wrap(g_seed);
    g_seed.with(|| x - 2000 + g_seed.get())
}

fn fpfi(pf: impl Fn(i64) -> i32, k: i32) -> impl Fn(i32, ...) -> i32 {
    let t = k as i64 * 97L + 1234L;
    let g_seed = std::thread::local!().wrap(g_seed);
    g_seed.with(|| pf(t) + k + g_seed.get())
}

fn call_through(pf: impl Fn(i32, ...) -> i32, x: i32) -> i32 {
    let r1 = pf(x);
    let r2 = pf(x, 1, 2);
    let r3 = pf(x, 1.25, std::mem::size_of::<()>() as u64, 0x1122334455667788LL);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let mut g_seed = std::thread::local::LocalKey::<i32>::new(0);

    let vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0((17 * 97L + 1234L) as i64) + 17;
        let base = expected_seed + 40;

        if std::ptr::eq(vf, &v0) {
            if vf(40) != base + 1000 {
                return 1;
            }
            if vf(40, 1, 2) != base + 1000 {
                return 2;
            }
            if call_through(vf, 40)
                != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9))
            {
                return 3;
            }
        } else {
            if vf(40) != base - 2000 {
                return 4;
            }
            if vf(40, 1, 2) != base - 2000 {
                return 5;
            }
            if call_through(vf, 40)
                != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9))
            {
                return 6;
            }
        }
    }

    let vf = fpfi(cb1, 8);

    {
        let expected_seed = cb1((8 * 97L + 1234L) as i64) + 8;
        let base = expected_seed + 9;

        if std::ptr::eq(vf, &v0) {
            if vf(9) != base + 1000 {
                return 7;
            }
            if vf(9, 0, 0, 0, 0) != base + 1000 {
                return 8;
            }
            if call_through(vf, 9)
                != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9))
            {
                return 9;
            }
        } else {
            if vf(9) != base - 2000 {
                return 10;
            }
            if vf(9, 0, 0, 0, 0) != base - 2000 {
                return 11;
            }
            if call_through(vf, 9)
                != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9))
            {
                return 12;
            }
        }
    }

    std::process::exit(0);
}