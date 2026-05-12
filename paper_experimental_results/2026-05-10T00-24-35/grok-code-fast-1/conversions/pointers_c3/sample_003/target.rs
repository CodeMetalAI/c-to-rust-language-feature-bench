fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 1;
    let px: Option<&i32> = Some(&x);
    let pf: Option<fn(i32) -> i32> = Some(f);

    let n1: Option<&i32> = None;
    let is_null_v0 = true;
    let n2: Option<&i32> = if is_null_v0 { None } else { Some(&x) };

    let nf1: Option<fn(i32) -> i32> = None;
    let is_null_vf0 = true;
    let nf2: Option<fn(i32) -> i32> = if is_null_vf0 { None } else { Some(f) };

    if n1.is_some() {
        std::process::exit(1);
    }
    if n2.is_some() {
        std::process::exit(2);
    }
    if !is_null_v0 {
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

    let is_null_vx = false;
    let is_null_vf = false;

    if is_null_vx {
        std::process::exit(10);
    }
    if is_null_vf {
        std::process::exit(11);
    }
    if is_null_vx == is_null_v0 {
        std::process::exit(12);
    }
    if is_null_vf == is_null_vf0 {
        std::process::exit(13);
    }

    std::process::exit(0);
}