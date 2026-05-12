use std::process;

#[derive(Clone, Copy)]
enum VarArg {
    Int(i32),
    Double(f64),
    Ptr(usize),
    LongLong(i64),
}

static G_SEED: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn cb0(x: i64) -> i32 {
    let v = (x ^ 0x13579) as i32;
    v.wrapping_add(7)
}

fn cb1(x: i64) -> i32 {
    let v = (x * 3) as i32;
    v.wrapping_sub(11)
}

fn v0(x: i32, _args: &[VarArg]) -> i32 {
    let seed = G_SEED.load(std::sync::atomic::Ordering::SeqCst);
    seed.wrapping_add(x).wrapping_add(1000)
}

fn v1(x: i32, _args: &[VarArg]) -> i32 {
    let seed = G_SEED.load(std::sync::atomic::Ordering::SeqCst);
    seed.wrapping_add(x).wrapping_sub(2000)
}

fn fpfi(pf: fn(i64) -> i32, k: i32) -> fn(i32, &[VarArg]) -> i32 {
    let t = (k as i64).wrapping_mul(97).wrapping_add(1234);
    let seed = pf(t).wrapping_add(k);
    G_SEED.store(seed, std::sync::atomic::Ordering::SeqCst);
    if (seed & 1) == 0 {
        v0
    } else {
        v1
    }
}

fn call_through(pf: fn(i32, &[VarArg]) -> i32, x: i32) -> i32 {
    let r1 = pf(x, &[]);
    let r2 = pf(
        x,
        &[
            VarArg::Int(1),
            VarArg::Int(2),
            VarArg::Int(3),
        ],
    );
    let r3 = pf(
        x,
        &[
            VarArg::Double(1.25),
            VarArg::Ptr(pf as usize),
            VarArg::LongLong(0x1122334455667788u64 as i64),
        ],
    );
    r1.wrapping_xor(r2.wrapping_add(5)).wrapping_xor(r3.wrapping_add(9))
}

fn main() {
    let mut exit_code = 0;
    let mut vf: fn(i32, &[VarArg]) -> i32;

    vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0((17i64).wrapping_mul(97).wrapping_add(1234)).wrapping_add(17);
        let base = expected_seed.wrapping_add(40);

        if vf as usize == v0 as usize {
            if vf(40, &[]) != base.wrapping_add(1000) {
                exit_code = 1;
            } else if vf(40, &[VarArg::Int(1), VarArg::Int(2)]) != base.wrapping_add(1000) {
                exit_code = 2;
            } else if call_through(vf, 40)
                != base
                    .wrapping_add(1000)
                    .wrapping_xor(base.wrapping_add(1000).wrapping_add(5))
                    .wrapping_xor(base.wrapping_add(1000).wrapping_add(9))
            {
                exit_code = 3;
            }
        } else {
            if vf(40, &[]) != base.wrapping_sub(2000) {
                exit_code = 4;
            } else if vf(40, &[VarArg::Int(1), VarArg::Int(2)]) != base.wrapping_sub(2000) {
                exit_code = 5;
            } else if call_through(vf, 40)
                != base
                    .wrapping_sub(2000)
                    .wrapping_xor(base.wrapping_sub(2000).wrapping_add(5))
                    .wrapping_xor(base.wrapping_sub(2000).wrapping_add(9))
            {
                exit_code = 6;
            }
        }
    }

    if exit_code == 0 {
        vf = fpfi(cb1, 8);

        let expected_seed = cb1((8i64).wrapping_mul(97).wrapping_add(1234)).wrapping_add(8);
        let base = expected_seed.wrapping_add(9);

        if vf as usize == v0 as usize {
            if vf(9, &[]) != base.wrapping_add(1000) {
                exit_code = 7;
            } else if vf(9, &[
                VarArg::Int(0),
                VarArg::Int(0),
                VarArg::Int(0),
                VarArg::Int(0),
            ]) != base.wrapping_add(1000)
            {
                exit_code = 8;
            } else if call_through(vf, 9)
                != base
                    .wrapping_add(1000)
                    .wrapping_xor(base.wrapping_add(1000).wrapping_add(5))
                    .wrapping_xor(base.wrapping_add(1000).wrapping_add(9))
            {
                exit_code = 9;
            }
        } else {
            if vf(9, &[]) != base.wrapping_sub(2000) {
                exit_code = 10;
            } else if vf(9, &[
                VarArg::Int(0),
                VarArg::Int(0),
                VarArg::Int(0),
                VarArg::Int(0),
            ]) != base.wrapping_sub(2000)
            {
                exit_code = 11;
            } else if call_through(vf, 9)
                != base
                    .wrapping_sub(2000)
                    .wrapping_xor(base.wrapping_sub(2000).wrapping_add(5))
                    .wrapping_xor(base.wrapping_sub(2000).wrapping_add(9))
            {
                exit_code = 12;
            }
        }
    }

    process::exit(exit_code);
}