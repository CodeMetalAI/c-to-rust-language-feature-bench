use std::mem::size_of;
use std::process::exit;

#[repr(C)]
struct St {
    a: i32,
    b: i32,
}

#[repr(C)]
union Un {
    i: i32,
    d: f64,
}

fn main() {
    let mut code = 0;

    let x = 0;
    let px1 = 0usize;
    let px2 = 0usize;
    if px1 != px2 {
        code = 1;
        exit(code);
    }

    let arr = [1, 2, 3];
    let p_end1 = arr.len();
    let p_end2 = 3usize;
    if p_end1 != p_end2 {
        code = 2;
        exit(code);
    }

    let p0 = 0usize;
    let p2 = 2usize;
    if !(p2 > p0) {
        code = 3;
        exit(code);
    }
    if !(p0 < p2) {
        code = 4;
        exit(code);
    }

    let q_last = 2usize;
    let q1 = q_last + 1;
    if !(q1 > p0) {
        code = 5;
        exit(code);
    }

    let _s = St { a: 0, b: 0 };
    let sa = 0usize;
    let sb = size_of::<i32>();
    if !(sb > sa) {
        code = 6;
        exit(code);
    }

    let _u = Un { i: 0 };
    let ui = 0usize;
    let ud = 0usize;
    if ui != ud {
        code = 7;
        exit(code);
    }

    exit(code);
}