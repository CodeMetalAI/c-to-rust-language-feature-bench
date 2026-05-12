fn main() {
    let x = 1;
    let px = &x;
    let pf = f as fn(i32) -> i32;

    let n1: Option<&i32> = None;
    let v0: Option<()> = None;
    let n2: Option<&i32> = None;

    let nf1: Option<fn(i32) -> i32> = None;
    let vf0: Option<()> = None;
    let nf2: Option<fn(i32) -> i32> = None;

    if n1.is_some() {
        std::process::exit(1);
    }
    if n2.is_some() {
        std::process::exit(2);
    }
    if v0.is_some() {
        std::process::exit(3);
    }

    if n1.is_some() && n1 == Some(px) {
        std::process::exit(4);
    }
    if n2.is_some() && n2 == Some(px) {
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

    let vx = v0 as *const _;
    let vf = vf0 as *const _;

    if vx.is_null() {
        std::process::exit(10);
    }
    if vf.is_null() {
        std::process::exit(11);
    }
    if std::ptr::eq(vx, &v0 as *const _) {
        std::process::exit(12);
    }
    if std::ptr::eq(vf, &vf0 as *const _) {
        std::process::exit(13);
    }
}

fn f(x: i32) -> i32 {
    x + 1
}