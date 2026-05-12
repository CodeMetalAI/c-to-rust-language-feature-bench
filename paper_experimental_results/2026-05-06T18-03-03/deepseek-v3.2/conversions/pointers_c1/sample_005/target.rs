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

    let v1: *const std::ffi::c_void = pi.cast();
    let v2: *const std::ffi::c_void = pd.cast();
    let v3: *const std::ffi::c_void = ps.cast();

    let pi2: *const i32 = v1.cast();
    let pd2: *const f64 = v2.cast();
    let ps2: *const S = v3.cast();

    if pi2 != pi {
        std::process::exit(1);
    }
    if pd2 != pd {
        std::process::exit(2);
    }
    if ps2 != ps {
        std::process::exit(3);
    }

    unsafe {
        if *pi2 != 42 {
            std::process::exit(4);
        }
        if *pd2 != 3.25 {
            std::process::exit(5);
        }
        if (*ps2).a != 7 {
            std::process::exit(6);
        }
        if (*ps2).b != 9.5 {
            std::process::exit(7);
        }
    }

    let v1b: *const std::ffi::c_void = pi2.cast();
    let v2b: *const std::ffi::c_void = pd2.cast();
    let v3b: *const std::ffi::c_void = ps2.cast();

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