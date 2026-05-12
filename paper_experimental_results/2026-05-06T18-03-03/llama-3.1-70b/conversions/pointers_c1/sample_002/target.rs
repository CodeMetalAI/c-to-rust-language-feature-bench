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

    if pi2 as usize != pi as usize {
        std::process::exit(1);
    }
    if pd2 as usize != pd as usize {
        std::process::exit(2);
    }
    if ps2 as usize != ps as usize {
        std::process::exit(3);
    }

    if unsafe { *pi2 } != 42 {
        std::process::exit(4);
    }
    if unsafe { *pd2 } != 3.25 {
        std::process::exit(5);
    }
    if unsafe { (*ps2).a } != 7 {
        std::process::exit(6);
    }
    if unsafe { (*ps2).b } != 9.5 {
        std::process::exit(7);
    }

    let v1b: *const () = pi2 as *const ();
    let v2b: *const () = pd2 as *const ();
    let v3b: *const () = ps2 as *const ();

    if v1b as usize != v1 as usize {
        std::process::exit(8);
    }
    if v2b as usize != v2 as usize {
        std::process::exit(9);
    }
    if v3b as usize != v3 as usize {
        std::process::exit(10);
    }
}

#[derive(Debug, PartialEq)]
struct S {
    a: i32,
    b: f64,
}