use std::process::exit;

#[repr(C)]
struct S {
    a: i32,
    b: f64,
}

fn main() {
    let xi: i32 = 42;
    let xd: f64 = 3.25;
    let xs = S { a: 7, b: 9.5 };

    let pi: &i32 = &xi;
    let pd: &f64 = &xd;
    let ps: &S = &xs;

    let v1: *const () = pi as *const i32 as *const ();
    let v2: *const () = pd as *const f64 as *const ();
    let v3: *const () = ps as *const S as *const ();

    let pi2: &i32 = pi;
    let pd2: &f64 = pd;
    let ps2: &S = ps;

    if !std::ptr::eq(pi2, pi) {
        exit(1);
    }
    if !std::ptr::eq(pd2, pd) {
        exit(2);
    }
    if !std::ptr::eq(ps2, ps) {
        exit(3);
    }

    if *pi2 != 42 {
        exit(4);
    }
    if *pd2 != 3.25 {
        exit(5);
    }
    if ps2.a != 7 {
        exit(6);
    }
    if ps2.b != 9.5 {
        exit(7);
    }

    let v1b: *const () = pi2 as *const i32 as *const ();
    let v2b: *const () = pd2 as *const f64 as *const ();
    let v3b: *const () = ps2 as *const S as *const ();

    if v1b != v1 {
        exit(8);
    }
    if v2b != v2 {
        exit(9);
    }
    if v3b != v3 {
        exit(10);
    }
}