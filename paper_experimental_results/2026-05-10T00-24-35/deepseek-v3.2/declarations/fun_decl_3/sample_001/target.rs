static mut G_SEED: i32 = 0;

fn cb0(x: i64) -> i32 {
    (x ^ 0x13579) as i32 + 7
}

fn cb1(x: i64) -> i32 {
    (x * 3) as i32 - 11
}

fn v0(x: i32, _: ...) -> i32 {
    G_SEED + x + 1000
}

fn v1(x: i32, _: ...) -> i32 {
    G_SEED + x - 2000
}

fn fpfi(pf: fn(i64) -> i32, k: i32) -> fn(i32, ...) -> i32 {
    let t = k as i64 * 97 + 1234;
    G_SEED = pf(t) + k;
    if (G_SEED & 1) == 0 {
        v0
    } else {
        v1
    }
}

fn call_through(pf: fn(i32, ...) -> i32, x: i32) -> i32 {
    let r1 = pf(x);
    let r2 = pf(x, 1, 2, 3);
    let r3 = pf(x, 1.25, pf as *const (), 0x1122334455667788);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let vf = fpfi(cb0, 17);

    let expected_seed = cb0(17i64 * 97 + 1234) + 17;
    let base = expected_seed + 40;

    if vf == v0 {
        if vf(40) != base + 1000 {
            return;
        }
        if vf(40, 1, 2) != base + 1000 {
            return;
        }
        if call_through(vf, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
            return;
        }
    } else {
        if vf(40) != base - 2000 {
            return;
        }
        if vf(40, 1, 2) != base - 2000 {
            return;
        }
        if call_through(vf, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
            return;
        }
    }

    let vf = fpfi(cb1, 8);

    let expected_seed = cb1(8i64 * 97 + 1234) + 8;
    let base = expected_seed + 9;

    if vf == v0 {
        if vf(9) != base + 1000 {
            return;
        }
        if vf(9, 0, 0, 0, 0) != base + 1000 {
            return;
        }
        if call_through(vf, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
            return;
        }
    } else {
        if vf(9) != base - 2000 {
            return;
        }
        if vf(9, 0, 0, 0, 0) != base - 2000 {
            return;
        }
        if call_through(vf, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
            return;
        }
    }
}