struct S {
    a: i32,
    b: f64,
}

fn main() {
    let xi = 42;
    let xd = 3.25;
    let xs = S { a: 7, b: 9.5 };

    let pi = &xi as *const i32;
    let pd = &xd as *const f64;
    let ps = &xs as *const S;

    let v1 = pi as *const ();
    let v2 = pd as *const ();
    let v3 = ps as *const ();

    let pi2 = v1 as *const i32;
    let pd2 = v2 as *const f64;
    let ps2 = v3 as *const S;

    if pi2 != pi {
        std::process::exit(1);
    }
    if pd2 != pd {
        std::process::exit(2);
    }
    if ps2 != ps {
        std::process::exit(3);
    }

    if xi != 42 {
        std::process::exit(4);
    }
    if xd != 3.25 {
        std::process::exit(5);
    }
    if xs.a != 7 {
        std::process::exit(6);
    }
    if xs.b != 9.5 {
        std::process::exit(7);
    }

    let v1b = pi2 as *const ();
    let v2b = pd2 as *const ();
    let v3b = ps2 as *const ();

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