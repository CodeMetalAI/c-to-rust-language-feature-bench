fn cb0(x: i64) -> i32 {
    (x ^ 0x13579) as i32 + 7
}

fn cb1(x: i64) -> i32 {
    (x * 3) as i32 - 11
}

fn v0(x: i32, _args: &[i32]) -> i32 {
    g_seed + x + 1000
}

fn v1(x: i32, _args: &[i32]) -> i32 {
    g_seed + x - 2000
}

static mut g_seed: i32 = 0;

type Callback = fn(i64) -> i32;
type VariadicFunc = fn(i32, &[i32]) -> i32;

fn fpfi(pf: Callback, k: i32) -> VariadicFunc {
    let t = (k as i64) * 97 + 1234;
    unsafe {
        g_seed = pf(t) + k;
    }
    if (unsafe { g_seed } & 1) == 0 {
        v0
    } else {
        v1
    }
}

fn call_through(pf: VariadicFunc, x: i32) -> i32 {
    let r1 = pf(x, &[]);
    let r2 = pf(x, &[1, 2, 3]);
    let r3 = pf(x, &[1, 2, 3, 4, 5]);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let mut vf = fpfi(cb0, 17);

    let expected_seed = cb0(17 as i64 * 97 + 1234);
    let base = expected_seed + 40;

    if vf == v0 {
        if vf(40, &[]) != base + 1000 {
            std::process::exit(1);
        }
        if vf(40, &[1, 2]) != base + 1000 {
            std::process::exit(2);
        }
        if call_through(vf, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
            std::process::exit(3);
        }
    } else {
        if vf(40, &[]) != base - 2000 {
            std::process::exit(4);
        }
        if vf(40, &[1, 2]) != base - 2000 {
            std::process::exit(5);
        }
        if call_through(vf, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
            std::process::exit(6);
        }
    }

    vf = fpfi(cb1, 8);

    let expected_seed = cb1(8 as i64 * 97 + 1234);
    let base = expected_seed + 9;

    if vf == v0 {
        if vf(9, &[]) != base + 1000 {
            std::process::exit(7);
        }
        if vf(9, &[0, 0, 0, 0]) != base + 1000 {
            std::process::exit(8);
        }
        if call_through(vf, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
            std::process::exit(9);
        }
    } else {
        if vf(9, &[]) != base - 2000 {
            std::process::exit(10);
        }
        if vf(9, &[0, 0, 0, 0]) != base - 2000 {
            std::process::exit(11);
        }
        if call_through(vf, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
            std::process::exit(12);
        }
    }

    std::process::exit(0);
}