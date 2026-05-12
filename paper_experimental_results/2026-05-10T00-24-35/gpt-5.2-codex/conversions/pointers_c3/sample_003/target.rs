use std::process::exit;

fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let _x = 1;
    let px: Option<usize> = Some(1); // id for x
    let pf: Option<usize> = Some(2); // id for f
    let _ = f; // reference to avoid unused function warnings

    let n1: Option<usize> = None;
    let v0: Option<usize> = None;
    let n2: Option<usize> = v0;

    let nf1: Option<usize> = None;
    let vf0: Option<usize> = None;
    let nf2: Option<usize> = vf0;

    if n1.is_some() {
        exit(1);
    }
    if n2.is_some() {
        exit(2);
    }
    if v0.is_some() {
        exit(3);
    }

    if n1 == px {
        exit(4);
    }
    if n2 == px {
        exit(5);
    }

    if nf1.is_some() {
        exit(6);
    }
    if nf2.is_some() {
        exit(7);
    }
    if nf1 == pf {
        exit(8);
    }
    if nf2 == pf {
        exit(9);
    }

    let vx: Option<usize> = px;
    let vf: Option<usize> = pf;

    if vx.is_none() {
        exit(10);
    }
    if vf.is_none() {
        exit(11);
    }
    if vx == v0 {
        exit(12);
    }
    if vf == vf0 {
        exit(13);
    }

    exit(0);
}