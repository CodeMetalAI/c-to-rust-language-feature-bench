fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x: i32 = 1;
    let px: Option<&i32> = Some(&x);
    let pf: Option<fn(i32) -> i32> = Some(f);

    let n1: Option<&i32> = None;
    let v0: Option<&()> = None;
    let n2: Option<&i32> = None;

    let nf1: Option<fn(i32) -> i32> = None;
    let vf0: Option<&()> = None;
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

    if n1 == px {
        std::process::exit(4);
    }
    if n2 == px {
        std::process::exit(5);
    }

    if nf1.is_some() {
        std::process::exit(6);
    }
    if nf2.is_some() {
        std::process::exit(7);
    }
    if nf1 == pf {
        std::process::exit(8);
    }
    if nf2 == pf {
        std::process::exit(9);
    }

    let vx: Option<&()> = if px.is_some() { Some(&()) } else { None };
    let vf: Option<&()> = if pf.is_some() { Some(&()) } else { None };

    if vx.is_none() {
        std::process::exit(10);
    }
    if vf.is_none() {
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