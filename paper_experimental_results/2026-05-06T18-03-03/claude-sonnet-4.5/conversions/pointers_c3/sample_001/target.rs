fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 1;
    let px: Option<&i32> = Some(&x);
    let pf: Option<fn(i32) -> i32> = Some(f);

    let n1: Option<&i32> = None;
    let v0: Option<*const ()> = None;
    let n2: Option<&i32> = None;

    let nf1: Option<fn(i32) -> i32> = None;
    let vf0: Option<*const ()> = None;
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

    if n1.is_some() && px.is_some() {
        std::process::exit(4);
    }
    if n2.is_some() && px.is_some() {
        std::process::exit(5);
    }

    if nf1.is_some() {
        std::process::exit(6);
    }
    if nf2.is_some() {
        std::process::exit(7);
    }
    if nf1.is_some() && pf.is_some() {
        std::process::exit(8);
    }
    if nf2.is_some() && pf.is_some() {
        std::process::exit(9);
    }

    let vx: Option<*const ()> = px.map(|p| p as *const i32 as *const ());
    let vf: Option<*const ()> = pf.map(|f| f as *const ());

    if vx.is_none() {
        std::process::exit(10);
    }
    if vf.is_none() {
        std::process::exit(11);
    }
    if vx.is_none() && v0.is_none() {
        std::process::exit(12);
    }
    if vf.is_none() && vf0.is_none() {
        std::process::exit(13);
    }

    std::process::exit(0);
}