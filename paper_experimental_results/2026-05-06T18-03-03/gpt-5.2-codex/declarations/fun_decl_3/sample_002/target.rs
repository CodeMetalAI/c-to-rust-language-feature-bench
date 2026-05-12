use std::sync::atomic::{AtomicI32, Ordering};
use std::process::exit;

static G_SEED: AtomicI32 = AtomicI32::new(0);

#[derive(Clone, Copy)]
enum Arg {
    Int(i32),
    Float(f64),
    Ptr(usize),
    LongLong(i64),
}

fn cb0(x: i64) -> i32 {
    let v = (x ^ 0x13579) as i32;
    v.wrapping_add(7)
}

fn cb1(x: i64) -> i32 {
    let v = (x * 3) as i32;
    v.wrapping_sub(11)
}

fn v0(x: i32, _args: &[Arg]) -> i32 {
    let seed = G_SEED.load(Ordering::Relaxed);
    seed.wrapping_add(x).wrapping_add(1000)
}

fn v1(x: i32, _args: &[Arg]) -> i32 {
    let seed = G_SEED.load(Ordering::Relaxed);
    seed.wrapping_add(x).wrapping_sub(2000)
}

type VarFn = fn(i32, &[Arg]) -> i32;

fn fpfi(pf: fn(i64) -> i32, k: i32) -> VarFn {
    let t = (k as i64) * 97 + 1234;
    let seed = pf(t).wrapping_add(k);
    G_SEED.store(seed, Ordering::Relaxed);
    if (seed & 1) == 0 {
        v0
    } else {
        v1
    }
}

fn call_through(pf: VarFn, x: i32) -> i32 {
    let r1 = pf(x, &[]);
    let args2 = [Arg::Int(1), Arg::Int(2), Arg::Int(3)];
    let r2 = pf(x, &args2);
    let args3 = [
        Arg::Float(1.25),
        Arg::Ptr(pf as usize),
        Arg::LongLong(0x1122334455667788),
    ];
    let r3 = pf(x, &args3);
    r1 ^ (r2.wrapping_add(5)) ^ (r3.wrapping_add(9))
}

fn main() {
    let mut vf: VarFn;

    vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0(17_i64 * 97 + 1234).wrapping_add(17);
        let base = expected_seed.wrapping_add(40);

        if vf == v0 {
            if vf(40, &[]) != base.wrapping_add(1000) {
                exit(1);
            }
            if vf(40, &[Arg::Int(1), Arg::Int(2)]) != base.wrapping_add(1000) {
                exit(2);
            }
            if call_through(vf, 40)
                != (base.wrapping_add(1000)
                    ^ base.wrapping_add(1000).wrapping_add(5)
                    ^ base.wrapping_add(1000).wrapping_add(9))
            {
                exit(3);
            }
        } else {
            if vf(40, &[]) != base.wrapping_sub(2000) {
                exit(4);
            }
            if vf(40, &[Arg::Int(1), Arg::Int(2)]) != base.wrapping_sub(2000) {
                exit(5);
            }
            if call_through(vf, 40)
                != (base.wrapping_sub(2000)
                    ^ base.wrapping_sub(2000).wrapping_add(5)
                    ^ base.wrapping_sub(2000).wrapping_add(9))
            {
                exit(6);
            }
        }
    }

    vf = fpfi(cb1, 8);

    {
        let expected_seed = cb1(8_i64 * 97 + 1234).wrapping_add(8);
        let base = expected_seed.wrapping_add(9);

        if vf == v0 {
            if vf(9, &[]) != base.wrapping_add(1000) {
                exit(7);
            }
            if vf(9, &[Arg::Int(0), Arg::Int(0), Arg::Int(0), Arg::Int(0)]) != base.wrapping_add(1000) {
                exit(8);
            }
            if call_through(vf, 9)
                != (base.wrapping_add(1000)
                    ^ base.wrapping_add(1000).wrapping_add(5)
                    ^ base.wrapping_add(1000).wrapping_add(9))
            {
                exit(9);
            }
        } else {
            if vf(9, &[]) != base.wrapping_sub(2000) {
                exit(10);
            }
            if vf(9, &[Arg::Int(0), Arg::Int(0), Arg::Int(0), Arg::Int(0)]) != base.wrapping_sub(2000) {
                exit(11);
            }
            if call_through(vf, 9)
                != (base.wrapping_sub(2000)
                    ^ base.wrapping_sub(2000).wrapping_add(5)
                    ^ base.wrapping_sub(2000).wrapping_add(9))
            {
                exit(12);
            }
        }
    }

    exit(0);
}