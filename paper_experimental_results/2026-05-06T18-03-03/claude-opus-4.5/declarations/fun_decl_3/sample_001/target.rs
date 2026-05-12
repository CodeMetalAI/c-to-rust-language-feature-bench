use std::cell::RefCell;

thread_local! {
    static G_SEED: RefCell<i32> = RefCell::new(0);
}

fn cb0(x: i64) -> i32 {
    (x ^ 0x13579_i64) as i32 + 7
}

fn cb1(x: i64) -> i32 {
    (x * 3_i64) as i32 - 11
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum VFunc {
    V0,
    V1,
}

fn v0(x: i32) -> i32 {
    G_SEED.with(|seed| *seed.borrow() + x + 1000)
}

fn v1(x: i32) -> i32 {
    G_SEED.with(|seed| *seed.borrow() + x - 2000)
}

fn call_vf(vf: VFunc, x: i32) -> i32 {
    match vf {
        VFunc::V0 => v0(x),
        VFunc::V1 => v1(x),
    }
}

fn fpfi<F>(pf: F, k: i32) -> VFunc
where
    F: Fn(i64) -> i32,
{
    let t = k as i64 * 97_i64 + 1234_i64;
    let new_seed = pf(t) + k;
    G_SEED.with(|seed| {
        *seed.borrow_mut() = new_seed;
    });
    if (new_seed & 1) == 0 {
        VFunc::V0
    } else {
        VFunc::V1
    }
}

fn call_through(vf: VFunc, x: i32) -> i32 {
    let r1 = call_vf(vf, x);
    let r2 = call_vf(vf, x);
    let r3 = call_vf(vf, x);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0(17_i64 * 97_i64 + 1234_i64) + 17;
        let base = expected_seed + 40;

        if vf == VFunc::V0 {
            if call_vf(vf, 40) != base + 1000 {
                std::process::exit(1);
            }
            if call_vf(vf, 40) != base + 1000 {
                std::process::exit(2);
            }
            if call_through(vf, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                std::process::exit(3);
            }
        } else {
            if call_vf(vf, 40) != base - 2000 {
                std::process::exit(4);
            }
            if call_vf(vf, 40) != base - 2000 {
                std::process::exit(5);
            }
            if call_through(vf, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                std::process::exit(6);
            }
        }
    }

    let vf = fpfi(cb1, 8);

    {
        let expected_seed = cb1(8_i64 * 97_i64 + 1234_i64) + 8;
        let base = expected_seed + 9;

        if vf == VFunc::V0 {
            if call_vf(vf, 9) != base + 1000 {
                std::process::exit(7);
            }
            if call_vf(vf, 9) != base + 1000 {
                std::process::exit(8);
            }
            if call_through(vf, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                std::process::exit(9);
            }
        } else {
            if call_vf(vf, 9) != base - 2000 {
                std::process::exit(10);
            }
            if call_vf(vf, 9) != base - 2000 {
                std::process::exit(11);
            }
            if call_through(vf, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                std::process::exit(12);
            }
        }
    }
}