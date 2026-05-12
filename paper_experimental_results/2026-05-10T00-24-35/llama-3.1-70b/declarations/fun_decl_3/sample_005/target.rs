fn cb0(x: i64) -> i32 {
    (x ^ 0x13579) as i32 + 7
}

fn cb1(x: i64) -> i32 {
    (x * 3) as i32 - 11
}

static mut G_SEED: i32 = 0;

type VFunc = dyn Fn(i32,...) -> i32;

fn v0(x: i32, _:...) -> i32 {
    unsafe { G_SEED + x + 1000 }
}

fn v1(x: i32, _:...) -> i32 {
    unsafe { G_SEED + x - 2000 }
}

fn fpfi(pf: fn(i64) -> i32, k: i32) -> &'static VFunc {
    let t = (k as i64) * 97 + 1234;
    unsafe {
        G_SEED = pf(t) + k;
        if (G_SEED & 1) == 0 {
            &v0 as &dyn Fn(i32,...) -> i32
        } else {
            &v1 as &dyn Fn(i32,...) -> i32
        }
    }
}

fn call_through(pf: &dyn Fn(i32,...) -> i32, x: i32) -> i32 {
    let r1 = pf(x);
    let r2 = pf(x, 1, 2, 3);
    let r3 = pf(x, 1.25, pf as *const (), 0x1122334455667788);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let vf: &dyn Fn(i32,...) -> i32 = fpfi(cb0, 17);

    let expected_seed = cb0(17 as i64 * 97 + 1234);
    let base = expected_seed + 40 + 17;

    if vf == &v0 as &dyn Fn(i32,...) -> i32 {
        if vf(40)!= base + 1000 {
            std::process::exit(1);
        }
        if vf(40, 1, 2)!= base + 1000 {
            std::process::exit(2);
        }
        if call_through(vf, 40)!= ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
            std::process::exit(3);
        }
    } else {
        if vf(40)!= base - 2000 {
            std::process::exit(4);
        }
        if vf(40, 1, 2)!= base - 2000 {
            std::process::exit(5);
        }
        if call_through(vf, 40)!= ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
            std::process::exit(6);
        }
    }

    let vf: &dyn Fn(i32,...) -> i32 = fpfi(cb1, 8);

    let expected_seed = cb1(8 as i64 * 97 + 1234);
    let base = expected_seed + 9 + 8;

    if vf == &v0 as &dyn Fn(i32,...) -> i32 {
        if vf(9)!= base + 1000 {
            std::process::exit(7);
        }
        if vf(9, 0, 0, 0, 0)!= base + 1000 {
            std::process::exit(8);
        }
        if call_through(vf, 9)!= ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
            std::process::exit(9);
        }
    } else {
        if vf(9)!= base - 2000 {
            std::process::exit(10);
        }
        if vf(9, 0, 0, 0, 0)!= base - 2000 {
            std::process::exit(11);
        }
        if call_through(vf, 9)!= ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
            std::process::exit(12);
        }
    }

    std::process::exit(0);
}