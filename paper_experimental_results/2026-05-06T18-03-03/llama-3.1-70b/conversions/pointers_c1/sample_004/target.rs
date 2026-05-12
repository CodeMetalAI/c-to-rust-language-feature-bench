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

    if pi2 as usize != pi as usize {
        return;
    }
    if pd2 as usize != pd as usize {
        return;
    }
    if ps2 as usize != ps as usize {
        return;
    }

    if unsafe { *pi2 } != 42 {
        return;
    }
    if unsafe { *pd2 } != 3.25 {
        return;
    }
    if unsafe { (*ps2).a } != 7 {
        return;
    }
    if unsafe { (*ps2).b } != 9.5 {
        return;
    }

    let v1b: *const () = pi2 as *const ();
    let v2b: *const () = pd2 as *const ();
    let v3b: *const () = ps2 as *const ();

    if v1b as usize != v1 as usize {
        return;
    }
    if v2b as usize != v2 as usize {
        return;
    }
    if v3b as usize != v3 as usize {
        return;
    }

    std::process::exit(0);
}