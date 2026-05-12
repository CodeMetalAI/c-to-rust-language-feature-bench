use std::mem::size_of;
use std::process::exit;

#[repr(C)]
struct St {
    a: i32,
    b: i32,
}

#[allow(dead_code)]
#[repr(C)]
union Un {
    i: i32,
    d: f64,
}

fn main() {
    let x = 0i32;
    let _ = x;
    let px1 = 0usize;
    let px2 = 0usize;
    if px1 != px2 {
        exit(1);
    }

    let arr = [1i32, 2, 3];
    let _ = arr;
    let p_end1 = 3usize;
    let p_end2 = 3usize;
    if p_end1 != p_end2 {
        exit(2);
    }

    let p0 = 0usize;
    let p2 = 2usize;
    if !(p2 > p0) {
        exit(3);
    }
    if !(p0 < p2) {
        exit(4);
    }

    let q_last = 2usize;
    let q1 = q_last + 1;
    if !(q1 > p0) {
        exit(5);
    }

    let _s = St { a: 0, b: 0 };
    let sa = 0usize;
    let sb = size_of::<i32>();
    if !(sb > sa) {
        exit(6);
    }

    let _u = Un { i: 0 };
    let ui = 0usize;
    let ud = 0usize;
    if ui != ud {
        exit(7);
    }

    exit(0);
}