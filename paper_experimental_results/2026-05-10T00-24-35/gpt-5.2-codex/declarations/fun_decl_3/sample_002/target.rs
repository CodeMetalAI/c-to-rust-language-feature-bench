use std::sync::atomic::{AtomicI32, Ordering};

static G_SEED: AtomicI32 = AtomicI32::new(0);

fn cb0(x: i64) -> i32 {
    let v = (x ^ 0x13579i64) as i32;
    v.wrapping_add(7)
}

fn cb1(x: i64) -> i32 {
    let v = x.wrapping_mul(3) as i32;
    v.wrapping_sub(11)
}

fn v0(x: i32) -> i32 {
    let seed = G_SEED.load(Ordering::SeqCst);
    seed.wrapping_add(x).wrapping_add(1000)
}

fn v1(x: i32) -> i32 {
    let seed = G_SEED.load(Ordering::SeqCst);
    seed.wrapping_add(x).wrapping_sub(2000)
}

fn fpfi(pf: fn(i64) -> i32, k: i32) -> fn(i32) -> i32 {
    let t = (k as i64).wrapping_mul(97).wrapping_add(1234);
    let seed = pf(t).wrapping_add(k);
    G_SEED.store(seed, Ordering::SeqCst);
    if (seed & 1) == 0 {
        v0
    } else {
        v1
    }
}

fn call_through(pf: fn(i32) -> i32, x: i32) -> i32 {
    let r1 = pf(x);
    let r2 = pf(x);
    let r3 = pf(x);
    r1 ^ r2.wrapping_add(5) ^ r3.wrapping_add(9)
}

fn main() {
    let mut vf: fn(i32) -> i32;

    vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0((17i64).wrapping_mul(97).wrapping_add(1234)).wrapping_add(17);
        let base = expected_seed.wrapping_add(40);

        if vf == v0 {
            if vf(40) != base.wrapping_add(1000) {
                std::process::exit(1);
            }
            if vf(40) != base.wrapping_add(1000) {
                std::process::exit(2);
            }
            let expected = base.wrapping_add(1000)
                ^ base.wrapping_add(1000).wrapping_add(5)
                ^ base.wrapping_add(1000).wrapping_add(9);
            if call_through(vf, 40) != expected {
                std::process::exit(3);
            }
        } else {
            if vf(40) != base.wrapping_sub(2000) {
                std::process::exit(4);
            }
            if vf(40) != base.wrapping_sub(2000) {
                std::process::exit(5);
            }
            let expected = base.wrapping_sub(2000)
                ^ base.wrapping_sub(2000).wrapping_add(5)
                ^ base.wrapping_sub(2000).wrapping_add(9);
            if call_through(vf, 40) != expected {
                std::process::exit(6);
            }
        }
    }

    vf = fpfi(cb1, 8);

    {
        let expected_seed = cb1((8i64).wrapping_mul(97).wrapping_add(1234)).wrapping_add(8);
        let base = expected_seed.wrapping_add(9);

        if vf == v0 {
            if vf(9) != base.wrapping_add(1000) {
                std::process::exit(7);
            }
            if vf(9) != base.wrapping_add(1000) {
                std::process::exit(8);
            }
            let expected = base.wrapping_add(1000)
                ^ base.wrapping_add(1000).wrapping_add(5)
                ^ base.wrapping_add(1000).wrapping_add(9);
            if call_through(vf, 9) != expected {
                std::process::exit(9);
            }
        } else {
            if vf(9) != base.wrapping_sub(2000) {
                std::process::exit(10);
            }
            if vf(9) != base.wrapping_sub(2000) {
                std::process::exit(11);
            }
            let expected = base.wrapping_sub(2000)
                ^ base.wrapping_sub(2000).wrapping_add(5)
                ^ base.wrapping_sub(2000).wrapping_add(9);
            if call_through(vf, 9) != expected {
                std::process::exit(12);
            }
        }
    }
}