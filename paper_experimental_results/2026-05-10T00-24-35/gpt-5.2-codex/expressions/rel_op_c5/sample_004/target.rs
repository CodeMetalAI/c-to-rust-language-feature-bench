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
    let px1_id = 0usize;
    let px2_id = 0usize;
    if px1_id != px2_id {
        exit(1);
    }

    let _arr = [1, 2, 3];
    let p_end1 = _arr.len();
    let p_end2 = _arr.len();
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
    let sb_gt_sa = true;
    if !sb_gt_sa {
        exit(6);
    }

    let _u = Un { i: 0 };
    let ui_eq_ud = true;
    if !ui_eq_ud {
        exit(7);
    }

    exit(0);
}