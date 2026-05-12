fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 1;
    let px = &x;
    let pf: fn(i32) -> i32 = f;

    let n1: *const i32 = std::ptr::null();
    let v0: *const () = std::ptr::null();
    let n2: *const i32 = v0 as *const i32;

    let nf1: fn(i32) -> i32 = std::ptr::null_fn;
    let vf0: *const () = std::ptr::null();
    let nf2: fn(i32) -> i32 = std::mem::transmute(vf0);

    if!n1.is_null() {
        std::process::exit(1);
    }
    if!n2.is_null() {
        std::process::exit(2);
    }
    if!v0.is_null() {
        std::process::exit(3);
    }

    if n1 as *const i32 == px {
        std::process::exit(4);
    }
    if n2 as *const i32 == px {
        std::process::exit(5);
    }

    if!nf1.is_null() {
        std::process::exit(6);
    }
    if!nf2.is_null() {
        std::process::exit(7);
    }
    if nf1 as *const () == pf as *const () {
        std::process::exit(8);
    }
    if nf2 as *const () == pf as *const () {
        std::process::exit(9);
    }

    let vx: *const () = px as *const ();
    let vf: *const () = pf as *const ();

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