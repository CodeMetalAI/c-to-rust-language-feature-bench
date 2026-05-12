struct S {
    a: i32,
    b: f64,
}

fn main() {
    let xi = 42;
    let xd = 3.25;
    let xs = S { a: 7, b: 9.5 };

    let pi = &xi;
    let pd = &xd;
    let ps = &xs;

    let v1 = pi as *const i32 as *const ();
    let v2 = pd as *const f64 as *const ();
    let v3 = ps as *const S as *const ();

    let pi2 = v1 as *const i32;
    let pd2 = v2 as *const f64;
    let ps2 = v3 as *const S;

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
    if ps2.as_ref().unwrap().a != 7 {
        return std::process::exit(6);
    }
    if ps2.as_ref().unwrap().b != 9.5 {
        return std::process::exit(7);
    }

    let v1b = pi2 as *const ();
    let v2b = pd2 as *const ();
    let v3b = ps2 as *const ();

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