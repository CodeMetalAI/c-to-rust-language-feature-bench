struct S {
    a: i32,
    b: f64,
}

fn main() -> i32 {
    let xi = 42;
    let xd = 3.25;
    let xs = S { a: 7, b: 9.5 };

    let pi: &i32 = &xi;
    let pd: &f64 = &xd;
    let ps: &S = &xs;

    let v1: *const i32 = pi as *const _;
    let v2: *const f64 = pd as *const _;
    let v3: *const S = ps as *const _;

    let pi2: &i32 = unsafe { &*v1 };
    let pd2: &f64 = unsafe { &*v2 };
    let ps2: &S = unsafe { &*v3 };

    if pi2 != pi {
        return 1;
    }
    if pd2 != pd {
        return 2;
    }
    if ps2 != ps {
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

    let v1b: *const i32 = pi2 as *const _;
    let v2b: *const f64 = pd2 as *const _;
    let v3b: *const S = ps2 as *const _;

    if v1b != v1 {
        return 8;
    }
    if v2b != v2 {
        return 9;
    }
    if v3b != v3 {
        return 10;
    }

    return 0;
}