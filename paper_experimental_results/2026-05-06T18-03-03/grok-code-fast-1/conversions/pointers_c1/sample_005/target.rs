#[derive(Debug)]
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

    if (pi2 as *const i32) != (pi as *const i32) {
        std::process::exit(1);
    }
    if (pd2 as *const f64) != (pd as *const f64) {
        std::process::exit(2);
    }
    if (ps2 as *const S) != (ps as *const S) {
        std::process::exit(3);
    }

    if *pi2 != 42 {
        std::process::exit(4);
    }
    if *pd2 != 3.25 {
        std::process::exit(5);
    }
    if ps2.a != 7 {
        std::process::exit(6);
    }
    if ps2.b != 9.5 {
        std::process::exit(7);
    }

    let v1b: *const () = pi2 as *const i32 as *const ();
    let v2b: *const () = pd2 as *const f64 as *const ();
    let v3b: *const () = ps2 as *const S as *const ();

    if v1b != v1 {
        std::process::exit(8);
    }
    if v2b != v2 {
        std::process::exit(9);
    }
    if v3b != v3 {
        std::process::exit(10);
    }

    std::process::exit(0);
}