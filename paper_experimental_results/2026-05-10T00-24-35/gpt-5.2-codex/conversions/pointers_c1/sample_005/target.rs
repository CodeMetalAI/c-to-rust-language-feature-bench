use std::any::Any;

struct S {
    a: i32,
    b: f64,
}

fn run() -> i32 {
    let xi: i32 = 42;
    let xd: f64 = 3.25;
    let xs = S { a: 7, b: 9.5 };

    let pi: &i32 = &xi;
    let pd: &f64 = &xd;
    let ps: &S = &xs;

    let v1: &dyn Any = pi;
    let v2: &dyn Any = pd;
    let v3: &dyn Any = ps;

    let pi2: &i32 = match v1.downcast_ref::<i32>() {
        Some(r) => r,
        None => return 1,
    };
    let pd2: &f64 = match v2.downcast_ref::<f64>() {
        Some(r) => r,
        None => return 2,
    };
    let ps2: &S = match v3.downcast_ref::<S>() {
        Some(r) => r,
        None => return 3,
    };

    if !std::ptr::eq(pi2, pi) {
        return 1;
    }
    if !std::ptr::eq(pd2, pd) {
        return 2;
    }
    if !std::ptr::eq(ps2, ps) {
        return 3;
    }

    if *pi2 != 42 {
        return 4;
    }
    if *pd2 != 3.25 {
        return 5;
    }
    if ps2.a != 7 {
        return 6;
    }
    if ps2.b != 9.5 {
        return 7;
    }

    let v1b: &dyn Any = pi2;
    let v2b: &dyn Any = pd2;
    let v3b: &dyn Any = ps2;

    if !std::ptr::eq(v1b, v1) {
        return 8;
    }
    if !std::ptr::eq(v2b, v2) {
        return 9;
    }
    if !std::ptr::eq(v3b, v3) {
        return 10;
    }

    0
}

fn main() {
    std::process::exit(run());
}