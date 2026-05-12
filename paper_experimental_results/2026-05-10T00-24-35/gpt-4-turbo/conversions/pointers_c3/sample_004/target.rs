fn main() {
    let x = 1;
    let px = &x;
    let pf = f;

    let n1: Option<&i32> = None;
    let v0: *const () = std::ptr::null();
    let n2: *const i32 = v0 as *const i32;

    let nf1: Option<fn(i32) -> i32> = None;
    let vf0: *const () = std::ptr::null();
    let nf2: fn(i32) -> i32 = unsafe { std::mem::transmute(vf0) };

    if n1.is_some() {
        std::process::exit(1);
    }
    if !n2.is_null() {
        std::process::exit(2);
    }
    if !v0.is_null() {
        std::process::exit(3);
    }

    if std::ptr::eq(px, n1.unwrap_or(std::ptr::null())) {
        std::process::exit(4);
    }
    if std::ptr::eq(px as *const i32, n2) {
        std::process::exit(5);
    }

    if nf1.is_some() {
        std::process::exit(6);
    }
    if !vf0.is_null() {
        std::process::exit(7);
    }
    if std::mem::transmute::<_, usize>(nf1.unwrap_or(std::mem::transmute(0usize))) == pf as usize {
        std::process::exit(8);
    }
    if std::mem::transmute::<_, usize>(nf2) == pf as usize {
        std::process::exit(9);
    }

    let vx = px as *const i32 as *const ();
    let vf = pf as *const ();

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
}

fn f(x: i32) -> i32 {
    x + 1
}