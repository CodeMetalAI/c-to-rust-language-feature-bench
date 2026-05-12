use std::process;

#[derive(Debug, Clone)]
struct S {
    a: i32,
    b: f64,
}

fn main() {
    let xi = 42;
    let xd = 3.25;
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
        process::exit(1);
    }
    if pd2 != pd {
        process::exit(2);
    }
    if ps2 != ps {
        process::exit(3);
    }

    unsafe {
        if *pi2 != 42 {
            process::exit(4);
        }
        if *pd2 != 3.25 {
            process::exit(5);
        }
        if (*ps2).a != 7 {
            process::exit(6);
        }
        if (*ps2).b != 9.5 {
            process::exit(7);
        }
    }

    let v1b: *const () = pi2 as *const ();
    let v2b: *const () = pd2 as *const ();
    let v3b: *const () = ps2 as *const ();

    if v1b != v1 {
        process::exit(8);
    }
    if v2b != v2 {
        process::exit(9);
    }
    if v3b != v3 {
        process::exit(10);
    }

    // Exit 0 implicitly
}