#[derive(Clone, Copy)]
struct S {
    a: i32,
    b: f64,
}

fn main() {
    let xi = 42i32;
    let xd = 3.25f64;
    let xs = S { a: 7, b: 9.5 };

    let addr_pi = &xi as *const i32 as usize;
    let addr_pd = &xd as *const f64 as usize;
    let addr_ps = &xs as *const S as usize;

    let v1 = addr_pi;
    let v2 = addr_pd;
    let v3 = addr_ps;

    let addr_pi2 = v1;
    let addr_pd2 = v2;
    let addr_ps2 = v3;

    if addr_pi2 != addr_pi {
        std::process::exit(1);
    }
    if addr_pd2 != addr_pd {
        std::process::exit(2);
    }
    if addr_ps2 != addr_ps {
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

    let v1b = addr_pi2;
    let v2b = addr_pd2;
    let v3b = addr_ps2;

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