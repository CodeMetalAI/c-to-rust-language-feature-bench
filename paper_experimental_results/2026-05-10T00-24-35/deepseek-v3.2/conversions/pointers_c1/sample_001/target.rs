struct S {
    a: i32,
    b: f64,
}

fn main() {
    let xi: i32 = 42;
    let xd: f64 = 3.25;
    let xs: S = S { a: 7, b: 9.5 };

    let pi: &i32 = &xi;
    let pd: &f64 = &xd;
    let ps: &S = &xs;

    let v1: *const i32 = pi as *const i32;
    let v2: *const f64 = pd as *const f64;
    let v3: *const S = ps as *const S;

    let pi2: *const i32 = v1;
    let pd2: *const f64 = v2;
    let ps2: *const S = v3;

    if pi2 != pi as *const i32 {
        return std::process::exit(1);
    }
    if pd2 != pd as *const f64 {
        return std::process::exit(2);
    }
    if ps2 != ps as *const S {
        return std::process::exit(3);
    }

    if *pi2 != 42 {
        return std::process::exit(4);
    }
    if *pd2 != 3.25 {
        return std::process::exit(5);
    }
    if unsafe { (*ps2).a } != 7 {
        return std::process::exit(6);
    }
    if unsafe { (*ps2).b } != 9.5 {
        return std::process::exit(7);
    }

    let v1b: *const i32 = pi2;
    let v2b: *const f64 = pd2;
    let v3b: *const S = ps2;

    if v1b != v1 {
        return std::process::exit(8);
    }
    if v2b != v2 {
        return std::process::exit(9);
    }
    if v3b != v3 {
        return std::process::exit(10);
    }

    std::process::exit(0);
}