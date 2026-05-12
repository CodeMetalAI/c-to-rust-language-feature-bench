use std::sync::atomic::{AtomicI32, Ordering};

static G_SEED: AtomicI32 = AtomicI32::new(0);

fn cb0(x: i64) -> i32 {
    (x ^ 0x13579) as i32 + 7
}

fn cb1(x: i64) -> i32 {
    (x * 3) as i32 - 11
}

fn v0(x: i32) -> i32 {
    G_SEED.load(Ordering::SeqCst) + x + 1000
}

fn v1(x: i32) -> i32 {
    G_SEED.load(Ordering::SeqCst) + x - 2000
}

type PfLong = fn(i64) -> i32;
type Vf = fn(i32) -> i32;

fn fpfi(pf: PfLong, k: i32) -> Vf {
    let t = k as i64 * 97 + 1234;
    let seed = pf(t) + k;
    G_SEED.store(seed, Ordering::SeqCst);
    if (seed & 1) == 0 {
        v0
    } else {
        v1
    }
}

fn call_through(pf: Vf, x: i32) -> i32 {
    let r1 = pf(x);
    let r2 = pf(x);
    let r3 = pf(x);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let vf = fpfi(cb0, 17);
    {
        let expected_seed = cb0(17i64 * 97 + 1234) + 17;
        let base = expected_seed + 40;

        if vf as usize == v0 as usize {
            if vf(40) != base + 1000 {
                std::process::exit(1);
            }
            if vf(40) != base + 1000 {
                std::process::exit(2);
            }
            if call_through(vf, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                std::process::exit(3);
            }
        } else {
            if vf(40) != base - 2000 {
                std::process::exit(4);
            }
            if vf(40) != base - 2000 {
                std::process::exit(5);
            }
            if call_through(vf, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                std::process::exit(6);
            }
        }
    }

    let vf = fpfi(cb1, 8);
    {
        let expected_seed = cb1(8i64 * 97 + 1234) + 8;
        let base = expected_seed + 9;

        if vf as usize == v0 as usize {
            if vf(9) != base + 1000 {
                std::process::exit(7);
            }
            if vf(9) != base + 1000 {
                std::process::exit(8);
            }
            if call_through(vf, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                std::process::exit(9);
            }
        } else {
            if vf(9) != base - 2000 {
                std::process::exit(10);
            }
            if vf(9) != base - 2000 {
                std::process::exit(11);
            }
            if call_through(vf, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                std::process::exit(12);
            }
        }
    }

    std::process::exit(0);
}