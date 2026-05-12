fn main() {
    let xi: i32 = 42;
    let xd: f64 = 3.25;
    let xs = S { a: 7, b: 9.5 };

    let pi: *const i32 = &xi;
    let pd: *const f64 = &xd;
    let ps: *const S = &xs;

    let v1: *const std::any::Any = pi as *const _;
    let v2: *const std::any::Any = pd as *const _;
    let v3: *const std::any::Any = ps as *const _;

    let pi2: *const i32 = v1 as *const i32;
    let pd2: *const f64 = v2 as *const f64;
    let ps2: *const S = v3 as *const S;

    if pi2 != pi {
        panic!("pi2 != pi");
    }
    if pd2 != pd {
        panic!("pd2 != pd");
    }
    if ps2 != ps {
        panic!("ps2 != ps");
    }

    if *pi2 != 42 {
        panic!("*pi2 != 42");
    }
    if *pd2 != 3.25 {
        panic!("*pd2 != 3.25");
    }
    if ps2.a != 7 {
        panic!("ps2.a != 7");
    }
    if ps2.b != 9.5 {
        panic!("ps2.b != 9.5");
    }

    let v1b: *const std::any::Any = pi2 as *const _;
    let v2b: *const std::any::Any = pd2 as *const _;
    let v3b: *const std::any::Any = ps2 as *const _;

    if v1b != v1 {
        panic!("v1b != v1");
    }
    if v2b != v2 {
        panic!("v2b != v2");
    }
    if v3b != v3 {
        panic!("v3b != v3");
    }

    println!("All tests passed.");
}

struct S {
    a: i32,
    b: f64,
}