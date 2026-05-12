fn f(x: i32) -> i32 {
    x + 1
}

fn main() -> i32 {
    let x = 1;
    let px: &i32 = &x;
    let pf: fn(i32) -> i32 = f;

    let n1: Option<&i32> = None;
    let v0: Option<*const ()> = None;
    let n2: Option<&i32> = v0.map(|_| std::ptr::null());

    let nf1: Option<fn(i32) -> i32> = None;
    let vf0: Option<*const ()> = None;
    let nf2: Option<fn(i32) -> i32> = vf0.map(|_| std::ptr::null());

    if n1.is_some() {
        return 1;
    }
    if n2.is_some() {
        return 2;
    }
    if v0.is_some() {
        return 3;
    }

    if n1.as_ref() == Some(px) {
        return 4;
    }
    if n2.as_ref() == Some(px) {
        return 5;
    }

    if nf1.is_some() {
        return 6;
    }
    if nf2.is_some() {
        return 7;
    }
    if nf1 == Some(pf) {
        return 8;
    }
    if nf2 == Some(pf) {
        return 9;
    }

    let vx: Option<*const i32> = Some(px as *const _);
    let vf: Option<*const fn(i32) -> i32> = Some(pf as *const _);

    if vx.is_none() {
        return 10;
    }
    if vf.is_none() {
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