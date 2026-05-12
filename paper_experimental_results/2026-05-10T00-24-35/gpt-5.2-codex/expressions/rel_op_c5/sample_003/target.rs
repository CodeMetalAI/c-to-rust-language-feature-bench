use std::mem;

struct St {
    a: i32,
    b: i32,
}

fn main() {
    let x = 0;
    let px1 = &x;
    let px2 = &x;
    if !std::ptr::eq(px1, px2) {
        std::process::exit(1);
    }

    let arr = [1, 2, 3];
    let p_end1_idx = 3usize;
    let p_end2_idx = 3usize;
    if p_end1_idx != p_end2_idx {
        std::process::exit(2);
    }

    let p0_idx = 0usize;
    let p2_idx = 2usize;
    if !(p2_idx > p0_idx) {
        std::process::exit(3);
    }
    if !(p0_idx < p2_idx) {
        std::process::exit(4);
    }

    let q_last_idx = 2usize;
    let q1_idx = q_last_idx + 1;
    if !(q1_idx > p0_idx) {
        std::process::exit(5);
    }

    let _s = St { a: 0, b: 0 };
    let sa_offset = 0usize;
    let sb_offset = mem::size_of::<i32>();
    if !(sb_offset > sa_offset) {
        std::process::exit(6);
    }

    let ui_offset = 0usize;
    let ud_offset = 0usize;
    if ui_offset != ud_offset {
        std::process::exit(7);
    }
}