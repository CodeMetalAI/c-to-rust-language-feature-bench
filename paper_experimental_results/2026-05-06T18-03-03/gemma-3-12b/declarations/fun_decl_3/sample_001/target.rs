static mut G_SEED: i32 = 0;

static fn cb0(x: i64) -> i32 {
    (x ^ 0x13579 as i64) as i32 + 7
}

static fn cb1(x: i64) -> i32 {
    (x * 3) as i32 - 11
}

fn v0(x: i32, args: manish::VarArgs) -> i32 {
    G_SEED + x + 1000
}

fn v1(x: i32, args: manish::VarArgs) -> i32 {
    G_SEED + x - 2000
}

fn fpfi(pf: fn(i64) -> i32, k: i32) -> fn(i32, manish::VarArgs) -> i32 {
    let t = (k as i64 * 97) + 1234;
    unsafe {
        G_SEED = pf(t) + k;
    }
    if (G_SEED & 1) == 0 {
        v0
    } else {
        v1
    }
}

fn call_through(pf: fn(i32, manish::VarArgs) -> i32, x: i32) -> i32 {
    let args0 = manish::VarArgs::new(x);
    let args1 = manish::VarArgs::new(x, 1, 2, 3);
    let args2 = manish::VarArgs::new(x, 1.25, Box::new(pf as *const fn(i32, manish::VarArgs) -> i32), 0x1122334455667788 as i64);

    let r1 = pf(x, args0);
    let r2 = pf(x, args1);
    let r3 = pf(x, args2);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

mod manish {
    pub struct VarArgs {
        args: Vec<Any>,
    }

    impl VarArgs {
        pub fn new(args: Any) -> Self {
            VarArgs { args: vec![args] }
        }

        pub fn new(args: Any, args2: i32, args3: i32, args4: i32) -> Self {
            VarArgs { args: vec![args, args2, args3, args4] }
        }

        pub fn new(args: Any, args2: f64, args3: Box<dyn Fn(i32, VarArgs) -> i32>, args4: i64) -> Self {
            VarArgs { args: vec![args, args2, args3, args4] }
        }
    }
}

use std::any::Any;

fn main() {
    let mut vf: fn(i32, manish::VarArgs) -> i32 = unsafe { fpfi(cb0, 17) };

    {
        let expected_seed = cb0((17 as i64 * 97) + 1234) + 17;
        let base = expected_seed + 40;

        if vf == v0 {
            if vf(40, manish::VarArgs::new(40)) != base + 1000 {
                return 1;
            }
            if vf(40, manish::VarArgs::new(40, 1, 2)) != base + 1000 {
                return 2;
            }
            if call_through(&vf, 40) !=
                ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                return 3;
            }
        } else {
            if vf(40, manish::VarArgs::new(40)) != base - 2000 {
                return 4;
            }
            if vf(40, manish::VarArgs::new(40, 1, 2)) != base - 2000 {
                return 5;
            }
            if call_through(&vf, 40) !=
                ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                return 6;
            }
        }
    }

    vf = unsafe { fpfi(cb1, 8) };

    {
        let expected_seed = cb1((8 as i64 * 97) + 1234) + 8;
        let base = expected_seed + 9;

        if vf == v0 {
            if vf(9, manish::VarArgs::new(9)) != base + 1000 {
                return 7;
            }
            if vf(9, manish::VarArgs::new(9, 0, 0, 0)) != base + 1000 {
                return 8;
            }
            if call_through(&vf, 9) !=
                ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                return 9;
            }
        } else {
            if vf(9, manish::VarArgs::new(9)) != base - 2000 {
                return 10;
            }
            if vf(9, manish::VarArgs::new(9, 0, 0, 0)) != base - 2000 {
                return 11;
            }
            if call_through(&vf, 9) !=
                ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                return 12;
            }
        }
    }

    0
}