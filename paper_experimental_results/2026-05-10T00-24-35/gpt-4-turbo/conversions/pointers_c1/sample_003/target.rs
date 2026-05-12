fn main() {
    let mut xi = 42;
    let mut xd = 3.25;
    let mut xs = S { a: 7, b: 9.5 };

    let pi = &mut xi as *mut i32;
    let pd = &mut xd as *mut f64;
    let ps = &mut xs as *mut S;

    let v1 = pi as *mut std::ffi::c_void;
    let v2 = pd as *mut std::ffi::c_void;
    let v3 = ps as *mut std::ffi::c_void;

    let pi2 = v1 as *mut i32;
    let pd2 = v2 as *mut f64;
    let ps2 = v3 as *mut S;

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

    let v1b = pi2 as *mut std::ffi::c_void;
    let v2b = pd2 as *mut std::ffi::c_void;
    let v3b = ps2 as *mut std::ffi::c_void;

    if v1b != v1 {
        std::process::exit(8);
    }
    if v2b != v2 {
        std::process::exit(9);
    }
    if v3b != v3 {
        std::process::exit(10);
    }
}

struct S {
    a: i32,
    b: f64,
}