#[derive(Debug, PartialEq)]
struct S {
    a: i32,
    b: f64,
}

fn main() {
    let xi = 42;
    let xd = 3.25;
    let xs = S { a: 7, b: 9.5 };

    let pi = &xi as *const i32 as *mut i32;
    let pd = &xd as *const f64 as *mut f64;
    let ps = &xs as *const S as *mut S;

    let v1: *mut i32 = pi as *mut i32;
    let v2: *mut f64 = pd as *mut f64;
    let v3: *mut S = ps as *mut S;

    let pi2 = v1 as *const i32;
    let pd2 = v2 as *const f64;
    let ps2 = v3 as *const S;

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

    let v1b = pi2 as *mut i32 as *void;
    let v2b = pd2 as *mut f64 as *void;
    let v3b = ps2 as *mut S as *void;

    if v1b != v1 as *void {
        return 8;
    }
    if v2b != v2 as *void {
        return 9;
    }
    if v3b != v3 as *void {
        return 10;
    }

    0
}