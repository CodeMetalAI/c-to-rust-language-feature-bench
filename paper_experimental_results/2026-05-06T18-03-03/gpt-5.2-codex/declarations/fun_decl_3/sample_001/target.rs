use std::cell::Cell;

thread_local! {
    static G_SEED: Cell<i32> = Cell::new(0);
}

fn get_seed() -> i32 {
    G_SEED.with(|c| c.get())
}

fn set_seed(v: i32) {
    G_SEED.with(|c| c.set(v));
}

#[derive(Clone, Copy)]
enum VarArg {
    Int(i32),
    Double(f64),
    Ptr(usize),
    LongLong(i64),
}

type VarFunc = fn(i32, &[VarArg]) -> i32;

fn cb0(x: i64) -> i32 {
    let v = (x ^ 0x13579) as i32;
    v.wrapping_add(7)
}

fn cb1(x: i64) -> i32 {
    let v = (x.wrapping_mul(3)) as i32;
    v.wrapping_sub(11)
}

fn v0(x: i32, _args: &[VarArg]) -> i32 {
    get_seed().wrapping_add(x).wrapping_add(1000)
}

fn v1(x: i32, _args: &[VarArg]) -> i32 {
    get_seed().wrapping_add(x).wrapping_sub(2000)
}

fn fpfi(pf: fn(i64) -> i32, k: i32) -> VarFunc {
    let t = (k as i64).wrapping_mul(97).wrapping_add(1234);
    let seed = pf(t).wrapping_add(k);
    set_seed(seed);
    if (seed & 1) == 0 {
        v0
    } else {
        v1
    }
}

fn call_through(pf: VarFunc, x: i32) -> i32 {
    let r1 = pf(x, &[]);
    let r2 = pf(x, &[VarArg::Int(1), VarArg::Int(2), VarArg::Int(3)]);
    let r3 = pf(
        x,
        &[
            VarArg::Double(1.25),
            VarArg::Ptr(pf as usize),
            VarArg::LongLong(0x1122334455667788u64 as i64),
        ],
    );
    r1 ^ r2.wrapping_add(5) ^ r3.wrapping_add(9)
}

fn main() {
    let mut vf: VarFunc;

    vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0((17i64).wrapping_mul(97).wrapping_add(1234)).wrapping_add(17);
        let base = expected_seed.wrapping_add(40);

        if vf == v0 {
            if vf(40, &[]) != base.wrapping_add(1000) {
                std::process::exit(1);
            }
            if vf(40, &[VarArg::Int(1), VarArg::Int(2)]) != base.wrapping_add(1000) {
                std::process::exit(2);
            }
            let expect = base.wrapping_add(1000)
                ^ base.wrapping_add(1000).wrapping_add(5)
                ^ base.wrapping_add(1000).wrapping_add(9);
            if call_through(vf, 40) != expect {
                std::process::exit(3);
            }
        } else {
            if vf(40, &[]) != base.wrapping_sub(2000) {
                std::process::exit(4);
            }
            if vf(40, &[VarArg::Int(1), VarArg::Int(2)]) != base.wrapping_sub(2000) {
                std::process::exit(5);
            }
            let expect = base.wrapping_sub(2000)
                ^ base.wrapping_sub(2000).wrapping_add(5)
                ^ base.wrapping_sub(2000).wrapping_add(9);
            if call_through(vf, 40) != expect {
                std::process::exit(6);
            }
        }
    }

    vf = fpfi(cb1, 8);

    {
        let expected_seed = cb1((8i64).wrapping_mul(97).wrapping_add(1234)).wrapping_add(8);
        let base = expected_seed.wrapping_add(9);

        if vf == v0 {
            if vf(9, &[]) != base.wrapping_add(1000) {
                std::process::exit(7);
            }
            if vf(9, &[VarArg::Int(0), VarArg::Int(0), VarArg::Int(0), VarArg::Int(0)])
                != base.wrapping_add(1000)
            {
                std::process::exit(8);
            }
            let expect = base.wrapping_add(1000)
                ^ base.wrapping_add(1000).wrapping_add(5)
                ^ base.wrapping_add(1000).wrapping_add(9);
            if call_through(vf, 9) != expect {
                std::process::exit(9);
            }
        } else {
            if vf(9, &[]) != base.wrapping_sub(2000) {
                std::process::exit(10);
            }
            if vf(9, &[VarArg::Int(0), VarArg::Int(0), VarArg::Int(0), VarArg::Int(0)])
                != base.wrapping_sub(2000)
            {
                std::process::exit(11);
            }
            let expect = base.wrapping_sub(2000)
                ^ base.wrapping_sub(2000).wrapping_add(5)
                ^ base.wrapping_sub(2000).wrapping_add(9);
            if call_through(vf, 9) != expect {
                std::process::exit(12);
            }
        }
    }

    std::process::exit(0);
}