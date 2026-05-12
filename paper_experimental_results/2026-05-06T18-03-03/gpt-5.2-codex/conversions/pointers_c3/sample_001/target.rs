use std::process::exit;

fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let _x = 1;
    let addr_x: usize = 1;
    let addr_f: usize = 2;

    let px: Option<usize> = Some(addr_x);
    let _pf: Option<usize> = Some(addr_f);

    let n1: Option<usize> = None;
    let v0: Option<usize> = None;
    let n2: Option<usize> = v0;

    let nf1: Option<usize> = None;
    let vf0: Option<usize> = None;
    let nf2: Option<usize> = vf0;

    if n1 != None {
        exit(1);
    }
    if n2 != None {
        exit(2);
    }
    if v0 != None {
        exit(3);
    }

    if n1 == px {
        exit(4);
    }
    if n2 == px {
        exit(5);
    }

    if nf1 != None {
        exit(6);
    }
    if nf2 != None {
        exit(7);
    }
    if nf1 == _pf {
        exit(8);
    }
    if nf2 == _pf {
        exit(9);
    }

    let vx: Option<usize> = px;
    let vf: Option<usize> = _pf;

    if vx == None {
        exit(10);
    }
    if vf == None {
        exit(11);
    }
    if vx == v0 {
        exit(12);
    }
    if vf == vf0 {
        exit(13);
    }

    let _ = f(0);
    exit(0);
}