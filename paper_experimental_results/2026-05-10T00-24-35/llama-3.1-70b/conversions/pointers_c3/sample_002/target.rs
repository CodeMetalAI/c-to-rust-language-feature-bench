fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x: i32 = 1;
    let px: *const i32 = &x;
    let pf: fn(i32) -> i32 = f;

    let n1: *const i32 = std::ptr::null();
    let v0: *const () = std::ptr::null();
    let n2: *const i32 = v0 as *const i32;

    let nf1: fn(i32) -> i32 = std::ptr::null_fn;
    let vf0: *const () = std::ptr::null();
    let nf2: fn(i32) -> i32 = std::mem::transmute(vf0);

    if n1 != std::ptr::null() {
        return;
    } else if n2 != std::ptr::null() {
        return;
    } else if v0 != std::ptr::null() {
        return;
    }

    if n1 as *const i32 == px {
        return;
    } else if n2 as *const i32 == px {
        return;
    }

    if nf1 as usize != 0 {
        return;
    } else if nf2 as usize != 0 {
        return;
    } else if nf1 as usize == pf as usize {
        return;
    } else if nf2 as usize == pf as usize {
        return;
    }

    let vx: *const () = px as *const ();
    let vf: *const () = pf as *const ();

    if vx.is_null() {
        return;
    } else if vf.is_null() {
        return;
    } else if vx == v0 {
        return;
    } else if vf == vf0 {
        return;
    }
}