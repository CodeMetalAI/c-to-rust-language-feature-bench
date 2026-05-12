use std::process::exit;

#[derive(Debug)]
struct S {
    a: i32,
    b: f64,
}

fn main() {
    let xi: i32 = 42;
    let xd: f64 = 3.25;
    let xs = S { a: 7, b: 9.5 };

    let pi: *const i32 = &xi;
    let pd: *const f64 = &xd;
    let ps: *const S = &xs;

    let v1: *const () = pi as *const ();
    let v2: *const () = pd as *const ();
    let v3: *const () = ps as *const ();

    let pi2: *const i32 = v1 as *const i32;
    let pd2: *const f64 = v2 as *const f64;
    let ps2: *const S = v3 as *const S;

    if pi2 != pi {
        exit(1);
    }
    if pd2 != pd {
        exit(2);
    }
    if ps2 != ps {
        exit(3);
    }

    if xi != 42 {
        exit(4);
    }
    if xd != 3.25 {
        exit(5);
    }
    if xs.a != 7 {
        exit(6);
    }
    if xs.b != 9.5 {
        exit(7);
    }

    let v1b: *const () = pi2 as *const ();
    let v2b: *const () = pd2 as *const ();
    let v3b: *const () = ps2 as *const ();

    if v1b != v1 {
        exit(8);
    }
    if v2b != v2 {
        exit(9);
    }
    if v3b != v3 {
        exit(10);
    }

    exit(0);
}