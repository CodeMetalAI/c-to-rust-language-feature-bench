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
    let nf2: fn(i32) -> i32 = unsafe { std::mem::transmute(vf0) };

    if!n1.is_null() {
        return;
    }
    if!n2.is_null() {
        return;
    }
    if!v0.is_null() {
        return;
    }

    if n1 == px {
        return;
    }
    if n2 == px {
        return;
    }

    if!nf1.is_null() {
        return;
    }
    if!nf2.is_null() {
        return;
    }
    if nf1 == pf {
        return;
    }
    if nf2 == pf {
        return;
    }

    let vx: *const () = px as *const ();
    let vf: *const () = pf as *const ();

    if vx.is_null() {
        return;
    }
    if vf.is_null() {
        return;
    }
    if vx == v0 {
        return;
    }
    if vf == vf0 {
        return;
    }

    // Exit with code 0
    std::process::exit(0);
}