fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x: i32 = 1;
    let px: Option<&i32> = Some(&x);
    let pf: Option<fn(i32) -> i32> = Some(f);

    let n1: Option<&i32> = None;
    let v0: Option<*const ()> = None;
    let n2: Option<&i32> = None;

    let nf1: Option<fn(i32) -> i32> = None;
    let _vf0: Option<*const ()> = None;
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

    // n1 == px would be false since n1 is None and px is Some
    if n1.is_some() && px.is_some() && std::ptr::eq(n1.unwrap(), px.unwrap()) {
        std::process::exit(4);
    }
    // n2 == px would be false since n2 is None and px is Some
    if n2.is_some() && px.is_some() && std::ptr::eq(n2.unwrap(), px.unwrap()) {
        std::process::exit(5);
    }

    if nf1.is_some() {
        std::process::exit(6);
    }
    if nf2.is_some() {
        std::process::exit(7);
    }
    // nf1 == pf would be false since nf1 is None
    if nf1.is_some() && pf.is_some() && nf1.unwrap() == pf.unwrap() {
        std::process::exit(8);
    }
    // nf2 == pf would be false since nf2 is None
    if nf2.is_some() && pf.is_some() && nf2.unwrap() == pf.unwrap() {
        std::process::exit(9);
    }

    let vx: Option<&i32> = px;
    let vf: Option<fn(i32) -> i32> = pf;

    if vx.is_none() {
        std::process::exit(10);
    }
    if vf.is_none() {
        std::process::exit(11);
    }
    // vx == v0 would be false since vx is Some and v0 is None
    if vx.is_none() && v0.is_none() {
        std::process::exit(12);
    }
    // vf == vf0 would be false since vf is Some and vf0 is None
    if vf.is_none() && _vf0.is_none() {
        std::process::exit(13);
    }

    std::process::exit(0);
}