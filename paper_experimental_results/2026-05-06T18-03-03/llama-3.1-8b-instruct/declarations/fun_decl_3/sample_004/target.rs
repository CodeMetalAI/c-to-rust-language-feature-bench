use std::thread;

static mut g_seed: i32 = 0;

fn cb0(x: i64) -> i32 {
    (x ^ 0x13579) as i32 + 7
}

fn cb1(x: i64) -> i32 {
    (x * 3) as i32 - 11
}

fn v0(x: i32, _args: ...) -> i32 {
    g_seed + x + 1000
}

fn v1(x: i32, _args: ...) -> i32 {
    g_seed + x - 2000
}

fn fpfi(pf: impl Fn(i64) -> i32, k: i32) -> impl Fn(i32, ...) -> i32 {
    let t = k as i64 * 97 + 1234;
    g_seed = pf(t) + k;
    if g_seed % 2 == 0 {
        v0
    } else {
        v1
    }
}

fn call_through(pf: impl Fn(i32, ...) -> i32, x: i32) -> i32 {
    let r1 = pf(x);
    let r2 = pf(x, 1, 2, 3);
    let r3 = pf(x, 1.25, std::mem::transmute(pf), 0x1122334455667788);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0((17 * 97) as i64 + 1234) + 17;
        let base = expected_seed + 40;

        if std::mem::discriminant(&vf) == std::mem::discriminant(&v0) {
            if vf(40) != base + 1000 {
                panic!("vf(40) != base + 1000");
            }
            if vf(40, 1, 2) != base + 1000 {
                panic!("vf(40, 1, 2) != base + 1000");
            }
            if call_through(vf, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                panic!("call_through(vf, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9))");
            }
        } else {
            if vf(40) != base - 2000 {
                panic!("vf(40) != base - 2000");
            }
            if vf(40, 1, 2) != base - 2000 {
                panic!("vf(40, 1, 2) != base - 2000");
            }
            if call_through(vf, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                panic!("call_through(vf, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9))");
            }
        }
    }

    let vf = fpfi(cb1, 8);

    {
        let expected_seed = cb1((8 * 97) as i64 + 1234) + 8;
        let base = expected_seed + 9;

        if std::mem::discriminant(&vf) == std::mem::discriminant(&v0) {
            if vf(9) != base + 1000 {
                panic!("vf(9) != base + 1000");
            }
            if vf(9, 0, 0, 0, 0) != base + 1000 {
                panic!("vf(9, 0, 0, 0, 0) != base + 1000");
            }
            if call_through(vf, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                panic!("call_through(vf, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9))");
            }
        } else {
            if vf(9) != base - 2000 {
                panic!("vf(9) != base - 2000");
            }
            if vf(9, 0, 0, 0, 0) != base - 2000 {
                panic!("vf(9, 0, 0, 0, 0) != base - 2000");
            }
            if call_through(vf, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                panic!("call_through(vf, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9))");
            }
        }
    }

    std::thread::park();
}