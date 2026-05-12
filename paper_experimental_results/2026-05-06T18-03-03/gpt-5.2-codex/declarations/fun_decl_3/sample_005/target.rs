use std::cell::Cell;
use std::process::exit;

#[derive(Debug)]
enum Arg {
    Int(i32),
    Double(f64),
    Ptr(usize),
    LongLong(i64),
}

type VariadicFn = fn(i32, &[Arg]) -> i32;
type PfLong = fn(i64) -> i32;

thread_local! {
    static G_SEED: Cell<i32> = Cell::new(0);
}

fn set_seed(val: i32) {
    G_SEED.with(|c| c.set(val));
}

fn get_seed() -> i32 {
    G_SEED.with(|c| c.get())
}

fn cb0(x: i64) -> i32 {
    let v = (x ^ 0x13579) as i32;
    v.wrapping_add(7)
}

fn cb1(x: i64) -> i32 {
    let v = (x.wrapping_mul(3)) as i32;
    v.wrapping_sub(11)
}

fn v0(x: i32, _args: &[Arg]) -> i32 {
    get_seed()
        .wrapping_add(x)
        .wrapping_add(1000)
}

fn v1(x: i32, _args: &[Arg]) -> i32 {
    get_seed()
        .wrapping_add(x)
        .wrapping_sub(2000)
}

fn fpfi(pf: PfLong, k: i32) -> VariadicFn {
    let t = (k as i64).wrapping_mul(97).wrapping_add(1234);
    let seed = pf(t).wrapping_add(k);
    set_seed(seed);
    if (seed & 1) == 0 {
        v0
    } else {
        v1
    }
}

fn call_through(pf: VariadicFn, x: i32) -> i32 {
    let r1 = pf(x, &[]);
    let r2 = pf(x, &[Arg::Int(1), Arg::Int(2), Arg::Int(3)]);
    let r3 = pf(
        x,
        &[
            Arg::Double(1.25),
            Arg::Ptr(pf as usize),
            Arg::LongLong(0x1122334455667788),
        ],
    );
    r1 ^ r2.wrapping_add(5) ^ r3.wrapping_add(9)
}

fn main() {
    let mut vf: VariadicFn;

    vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0((17i64).wrapping_mul(97).wrapping_add(1234))
            .wrapping_add(17);
        let base = expected_seed.wrapping_add(40);

        if vf == v0 {
            if vf(40, &[]) != base.wrapping_add(1000) {
                exit(1);
            }
            if vf(40, &[Arg::Int(1), Arg::Int(2)]) != base.wrapping_add(1000) {
                exit(2);
            }
            let expected = base
                .wrapping_add(1000)
                ^ base.wrapping_add(1000).wrapping_add(5)
                ^ base.wrapping_add(1000).wrapping_add(9);
            if call_through(vf, 40) != expected {
                exit(3);
            }
        } else {
            if vf(40, &[]) != base.wrapping_sub(2000) {
                exit(4);
            }
            if vf(40, &[Arg::Int(1), Arg::Int(2)]) != base.wrapping_sub(2000) {
                exit(5);
            }
            let expected = base
                .wrapping_sub(2000)
                ^ base.wrapping_sub(2000).wrapping_add(5)
                ^ base.wrapping_sub(2000).wrapping_add(9);
            if call_through(vf, 40) != expected {
                exit(6);
            }
        }
    }

    vf = fpfi(cb1, 8);

    {
        let expected_seed = cb1((8i64).wrapping_mul(97).wrapping_add(1234)).wrapping_add(8);
        let base = expected_seed.wrapping_add(9);

        if vf == v0 {
            if vf(9, &[]) != base.wrapping_add(1000) {
                exit(7);
            }
            if vf(9, &[Arg::Int(0), Arg::Int(0), Arg::Int(0), Arg::Int(0)])
                != base.wrapping_add(1000)
            {
                exit(8);
            }
            let expected = base
                .wrapping_add(1000)
                ^ base.wrapping_add(1000).wrapping_add(5)
                ^ base.wrapping_add(1000).wrapping_add(9);
            if call_through(vf, 9) != expected {
                exit(9);
            }
        } else {
            if vf(9, &[]) != base.wrapping_sub(2000) {
                exit(10);
            }
            if vf(9, &[Arg::Int(0), Arg::Int(0), Arg::Int(0), Arg::Int(0)])
                != base.wrapping_sub(2000)
            {
                exit(11);
            }
            let expected = base
                .wrapping_sub(2000)
                ^ base.wrapping_sub(2000).wrapping_add(5)
                ^ base.wrapping_sub(2000).wrapping_add(9);
            if call_through(vf, 9) != expected {
                exit(12);
            }
        }
    }

    exit(0);
}