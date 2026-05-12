use std::cell::RefCell;

fn cb0(x: i64) -> i32 {
    (x ^ 0x13579) as i32 + 7
}

fn cb1(x: i64) -> i32 {
    (x * 3) as i32 - 11
}

fn v0(x: i32, ...: ()) -> i32 {
    let mut g_seed = RefCell::new(0);
    g_seed.borrow_mut() + x + 1000
}

fn v1(x: i32, ...: ()) -> i32 {
    let mut g_seed = RefCell::new(0);
    g_seed.borrow_mut() + x - 2000
}

fn fpfi(pf: impl Fn(i64) -> i32, k: i32) -> impl Fn(i32, ...) -> i32 {
    let t = k as i64 * 97 + 1234;
    let mut g_seed = RefCell::new(0);
    let seed = pf(t) + k;
    g_seed.borrow_mut() = seed;
    if seed & 1 == 0 {
        v0
    } else {
        v1
    }
}

fn call_through(pf: impl Fn(i32, ...) -> i32, x: i32) -> i32 {
    let r1 = pf(x);
    let r2 = pf(x, 1, 2, 3);
    let r3 = pf(x, 1.25, &pf, 0x1122334455667788);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0((17 * 97 + 1234) as i64);
        let base = expected_seed + 40;

        if vf == v0 {
            if vf(40) != base + 1000 {
                return 1;
            }
            if vf(40, 1, 2) != base + 1000 {
                return 2;
            }
            if call_through(vf, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                return 3;
            }
        } else {
            if vf(40) != base - 2000 {
                return 4;
            }
            if vf(40, 1, 2) != base - 2000 {
                return 5;
            }
            if call_through(vf, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                return 6;
            }
        }
    }

    let vf = fpfi(cb1, 8);

    {
        let expected_seed = cb1((8 * 97 + 1234) as i64);
        let base = expected_seed + 9;

        if vf == v0 {
            if vf(9) != base + 1000 {
                return 7;
            }
            if vf(9, 0, 0, 0, 0) != base + 1000 {
                return 8;
            }
            if call_through(vf, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                return 9;
            }
        } else {
            if vf(9) != base - 2000 {
                return 10;
            }
            if vf(9, 0, 0, 0, 0) != base - 2000 {
                return 11;
            }
            if call_through(vf, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                return 12;
            }
        }
    }

    0
}