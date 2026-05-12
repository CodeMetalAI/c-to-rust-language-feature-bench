fn main() {
    let x = 1;
    let px = &x;
    let pf = f;

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

    if matches!(n1, Some(p) if std::ptr::eq(p, px)) {
        std::process::exit(4);
    }
    if matches!(n2, Some(p) if std::ptr::eq(p, px)) {
        std::process::exit(5);
    }

    if nf1.is_some() {
        std::process::exit(6);
    }
    if nf2.is_some() {
        std::process::exit(7);
    }
    if matches!(nf1, Some(f) if std::ptr::eq(f as *const (), pf as *const ())) {
        std::process::exit(8);
    }
    if matches!(nf2, Some(f) if std::ptr::eq(f as *const (), pf as *const ())) {
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
    if vx == v0.map(|_| std::ptr::null()).unwrap_or(std::ptr::null()) {
        std::process::exit(12);
    }
    if vf == vf0.map(|_| std::ptr::null()).unwrap_or(std::ptr::null()) {
        std::process::exit(13);
    }
}

fn f(x: i32) -> i32 {
    x + 1
}