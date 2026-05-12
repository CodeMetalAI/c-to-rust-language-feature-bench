struct S {
    a: i32,
    b: f64,
}

fn main() {
    let xi: i32 = 42;
    let xd: f64 = 3.25;
    let xs = S { a: 7, b: 9.5 };

    let pi = &xi;
    let pd = &xd;
    let ps = &xs;

    // In safe Rust, we can't have void pointers, but we can verify
    // that references work correctly by going through the same logical flow
    
    let pi2 = pi;
    let pd2 = pd;
    let ps2 = ps;

    // Check that references point to the same data (in Rust, these are the same references)
    if !std::ptr::eq(pi2, pi) {
        std::process::exit(1);
    }
    if !std::ptr::eq(pd2, pd) {
        std::process::exit(2);
    }
    if !std::ptr::eq(ps2, ps) {
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

    // Verify the references are still equivalent after reassignment
    let pi2b = pi2;
    let pd2b = pd2;
    let ps2b = ps2;

    if !std::ptr::eq(pi2b, pi) {
        std::process::exit(8);
    }
    if !std::ptr::eq(pd2b, pd) {
        std::process::exit(9);
    }
    if !std::ptr::eq(ps2b, ps) {
        std::process::exit(10);
    }

    std::process::exit(0);
}