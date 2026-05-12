use std::sync::atomic::{AtomicI32, Ordering};
use std::process::exit;

static G_SEED: AtomicI32 = AtomicI32::new(0);

fn cb0(x: i64) -> i32 {
    ((x ^ 0x13579) as i32) + 7
}

fn cb1(x: i64) -> i32 {
    (x as i32 * 3) - 11
}

#[derive(Clone, Copy)]
enum Arg {
    I32(i32),
    F64(f64),
    Ptr(usize),
    I64(i64),
}

fn v0(x: i32, _args: &[Arg]) -> i32 {
    G_SEED.load(Ordering::SeqCst) + x + 1000
}

fn v1(x: i32, _args: &[Arg]) -> i32 {
    G_SEED.load(Ordering::SeqCst) + x - 2000
}

fn fpfi(pf: fn(i64) -> i32, k: i32) -> fn(i32, &[Arg]) -> i32 {
    let t = (k as i64) * 97 + 1234;
    let seed = pf(t) + k;
    G_SEED.store(seed, Ordering::SeqCst);
    if (seed & 1) == 0 {
        v0
    } else {
        v1
    }
}

fn call_through(pf: fn(i32, &[Arg]) -> i32, x: i32) -> i32 {
    let r1 = pf(x, &[]);
    let r2 = pf(x, &[Arg::I32(1), Arg::I32(2), Arg::I32(3)]);
    let r3 = pf(
        x,
        &[
            Arg::F64(1.25),
            Arg::Ptr(pf as usize),
            Arg::I64(0x1122334455667788),
        ],
    );
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let mut vf: fn(i32, &[Arg]) -> i32;

    vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0(17 * 97 + 1234) + 17;
        let base = expected_seed + 40;

        if vf == v0 {
            if vf(40, &[]) != base + 1000 {
                exit(1);
            }
            if vf(40, &[Arg::I32(1), Arg::I32(2)]) != base + 1000 {
                exit(2);
            }
            if call_through(vf, 40)
                != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9))
            {
                exit(3);
            }
        } else {
            if vf(40, &[]) != base - 2000 {
                exit(4);
            }
            if vf(40, &[Arg::I32(1), Arg::I32(2)]) != base - 2000 {
                exit(5);
            }
            if call_through(vf, 40)
                != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9))
            {
                exit(6);
            }
        }
    }

    vf = fpfi(cb1, 8);

    {
        let expected_seed = cb1(8 * 97 + 1234) + 8;
        let base = expected_seed + 9;

        if vf == v0 {
            if vf(9, &[]) != base + 1000 {
                exit(7);
            }
            if vf(9, &[Arg::I32(0), Arg::I32(0), Arg::I32(0), Arg::I32(0)]) != base + 1000 {
                exit(8);
            }
            if call_through(vf, 9)
                != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9))
            {
                exit(9);
            }
        } else {
            if vf(9, &[]) != base - 2000 {
                exit(10);
            }
            if vf(9, &[Arg::I32(0), Arg::I32(0), Arg::I32(0), Arg::I32(0)]) != base - 2000 {
                exit(11);
            }
            if call_through(vf, 9)
                != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9))
            {
                exit(12);
            }
        }
    }

    exit(0);
}