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
    let _x = 0;
    let px1_idx = 0;
    let px2_idx = 0;
    if px1_idx != px2_idx {
        exit(1);
    }

    let _arr = [1, 2, 3];
    let p_end1_idx = 3;
    let p_end2_idx = 3;
    if p_end1_idx != p_end2_idx {
        exit(2);
    }

    let p0_idx = 0;
    let p2_idx = 2;
    if !(p2_idx > p0_idx) {
        exit(3);
    }
    if !(p0_idx < p2_idx) {
        exit(4);
    }

    let q_last_idx = 2;
    let q1_idx = q_last_idx + 1;
    if !(q1_idx > p0_idx) {
        exit(5);
    }

    let _s = St { a: 0, b: 0 };
    let sa_off = std::mem::offset_of!(St, a);
    let sb_off = std::mem::offset_of!(St, b);
    if !(sb_off > sa_off) {
        exit(6);
    }

    let _u = Un { i: 0 };
    let ui_off = std::mem::offset_of!(Un, i);
    let ud_off = std::mem::offset_of!(Un, d);
    if ui_off != ud_off {
        exit(7);
    }

    exit(0);
}