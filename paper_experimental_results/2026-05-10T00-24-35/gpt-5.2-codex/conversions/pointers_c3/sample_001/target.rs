#[derive(Clone, Copy, PartialEq, Eq)]
struct Ptr(i32);

fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let _x = 1;
    let px = Ptr(1);
    let _pf_fn: fn(i32) -> i32 = f;
    let pf = Ptr(2);

    let n1: Option<Ptr> = None;
    let v0: Option<Ptr> = None;
    let n2: Option<Ptr> = v0;

    let nf1: Option<Ptr> = None;
    let vf0: Option<Ptr> = None;
    let nf2: Option<Ptr> = vf0;

    let code = if n1.is_some() {
        1
    } else if n2.is_some() {
        2
    } else if v0.is_some() {
        3
    } else if n1 == Some(px) {
        4
    } else if n2 == Some(px) {
        5
    } else if nf1.is_some() {
        6
    } else if nf2.is_some() {
        7
    } else if nf1 == Some(pf) {
        8
    } else if nf2 == Some(pf) {
        9
    } else {
        let vx: Option<Ptr> = Some(px);
        let vf: Option<Ptr> = Some(pf);

        if vx.is_none() {
            10
        } else if vf.is_none() {
            11
        } else if vx == v0 {
            12
        } else if vf == vf0 {
            13
        } else {
            0
        }
    };

    std::process::exit(code);
}