static mut g_seed: i32 = 0;

fn cb0(x: i32) -> i32 {
    x ^ 0x13579 as i32 + 7
}

fn cb1(x: i32) -> i32 {
    (x * 3) - 11
}

fn v0(x: i32, _args: ...) -> i32 {
    g_seed + x + 1000
}

fn v1(x: i32, _args: ...) -> i32 {
    g_seed + x - 2000
}

fn fpfi(pf: fn(i32) -> i32, k: i32) -> fn(i32, ...) {
    let t = (k as i64 * 97 + 1234) as i32;
    g_seed = pf(t) + k;
    if g_seed & 1 == 0 {
        v0
    } else {
        v1
    }
}

fn call_through(pf: fn(i32, ...) -> i32, x: i32) -> i32 {
    let r1 = pf(x);
    let r2 = pf(x, 1, 2, 3);
    let r3 = pf(x, 1.25, std::ptr::null(), 0x1122334455667788 as i64);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let mut vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0((17 * 97 + 1234) as i32) + 17;
        let base = expected_seed + 40;

        if vf == v0 {
            if vf(40) != base + 1000 {
                return 1;
            }
            if vf(40, 1, 2) != base + 1000 {
                return 2;
            }
            if call_through(vf, 40) != ((base + 1000) ^ (base + 1005) ^ (base + 1009)) {
                return 3;
            }
        } else {
            if vf(40) != base - 2000 {
                return 4;
            }
            if vf(40, 1, 2) != base - 2000 {
                return 5;
            }
            if call_through(vf, 40) != ((base - 2000) ^ (base - 2005) ^ (base - 2009)) {
                return 6;
            }
        }
    }

    vf = fpfi(cb1, 8);

    {
        let expected_seed = cb1((8 * 97 + 1234) as i32) + 8;
        let base = expected_seed + 9;

        if vf == v0 {
            if vf(9) != base + 1000 {
                return 7;
            }
            if vf(9, 0, 0, 0, 0) != base + 1000 {
                return 8;
            }
            if call_through(vf, 9) != ((base + 1000) ^ (base + 1005) ^ (base + 1009)) {
                return 9;
            }
        } else {
            if vf(9) != base - 2000 {
                return 10;
            }
            if vf(9, 0, 0, 0, 0) != base - 2000 {
                return 11;
            }
            if call_through(vf, 9) != ((base - 2000) ^ (base - 2005) ^ (base - 2009)) {
                return 12;
            }
        }
    }

    0
}