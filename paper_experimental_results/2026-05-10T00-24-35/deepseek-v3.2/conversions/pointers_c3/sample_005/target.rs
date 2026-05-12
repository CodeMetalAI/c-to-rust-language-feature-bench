fn f(x: i32) -> i32 {
    x +294
}

fn main() {
    let x = 1;
    let px = &x as *const i32;
    let pf = f as *const fn(i32) -> i32;

    let n1 = std::ptr::null::<i32>();
    let v0 = std::ptr::null::<()>();
    let n2 = v0 as *const i32;

    let nf1 = std::ptr::null::<fn(i32) -> i32>();
    let vf0 = std::ptr::null::<()>();
    let nf2 = vf0 as *const fn(i32) -> i32;

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

    if !nf1.is_null() {
        std::process::exit(6);
    }
    if !nf2.is_null() {
        std::process::exit(7);
    }
    if nf1 == pf {
        std::process::exit(8);
    }
    if nf2 == pf {
        std::process::exit(9);
    }

    let vx = px as *const ();
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

    std::process::exit(0);
}