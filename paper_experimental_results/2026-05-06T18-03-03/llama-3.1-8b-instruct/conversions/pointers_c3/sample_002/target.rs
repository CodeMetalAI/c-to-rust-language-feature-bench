fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 1;
    let px = &x;
    let pf = f;

    let n1: *const i32 = std::ptr::null();
    let v0: *const () = std::ptr::null();
    let n2 = v0 as *const i32;

    let nf1: Option<&dyn Fn(i32) -> i32> = None;
    let vf0: *const () = std::ptr::null();
    let nf2 = vf0 as *const dyn Fn(i32) -> i32;

    if n1 != std::ptr::null() {
        return 1;
    }
    if n2 != std::ptr::null() {
        return 2;
    }
    if v0 != std::ptr::null() {
        return 3;
    }

    if n1 == px as *const i32 {
        return 4;
    }
    if n2 == px as *const i32 {
        return 5;
    }

    if nf1.is_some() {
        return 6;
    }
    if nf2 != std::ptr::null() {
        return 7;
    }
    if nf1 == Some(&f) {
        return 8;
    }
    if nf2 == Some(&f) {
        return 9;
    }

    let vx = px as *const ();
    let vf = pf as *const ();

    if vx == std::ptr::null() {
        return 10;
    }
    if vf == std::ptr::null() {
        return 11;
    }
    if vx == v0 {
        return 12;
    }
    if vf == vf0 {
        return 13;
    }

    0
}