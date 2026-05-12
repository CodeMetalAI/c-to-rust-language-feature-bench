fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 1;
    let px = &x as *const i32;
    let pf: fn(i32) -> i32 = f;

    let n1: *const i32 = std::ptr::null();
    let v0: *const () = std::ptr::null();
    let n2: *const i32 = v0 as *const i32;

    let nf1: Option<fn(i32) -> i32> = None;
    let vf0: *const () = std::ptr::null();
    let nf2: Option<fn(i32) -> i32> = unsafe { std::mem::transmute(vf0) };

    if !n1.is_null() {
        std::process::exit(1);
    }
    if !n2.is_null() {
        std::process::exit(2);
    }
    if !v0.is_null() {
        std::process::exit(3);
    }

    if n1 == px {
        std::process::exit(4);
    }
    if n2 == px {
        std::process::exit(5);
    }

    if nf1.is_some() {
        std::process::exit(6);
    }
    if nf2.is_some() {
        std::process::exit(7);
    }
    if nf1 == Some(pf) {
        std::process::exit(8);
    }
    if nf2 == Some(pf) {
        std::process::exit(9);
    }

    let vx: *const () = px as *const ();
    let vf: *const () = unsafe { std::mem::transmute(pf) };

    if vx.is_null() {
        std::process::exit(10);
    }
    if vf.is_null() {
        std::process::exit(11);
    }
    if vx == v0 {
        std::process::exit(12);
    }
    if vf == vf0 {
        std::process::exit(13);
    }

    std::process::exit(0);
}