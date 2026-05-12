use std::process::exit;
use std::mem::{align_of, size_of};

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

fn align_up(offset: usize, align: usize) -> usize {
    if align == 0 {
        offset
    } else {
        ((offset + align - 1) / align) * align
    }
}

fn main() {
    let x = 0;
    let px1_id = 0usize;
    let px2_id = 0usize;
    if px1_id != px2_id {
        exit(1);
    }

    let arr = [1, 2, 3];
    let p_end1 = arr.len();
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
    let sa_offset = 0usize;
    let sb_offset = align_up(size_of::<i32>(), align_of::<i32>());
    if !(sb_offset > sa_offset) {
        exit(6);
    }

    let _u = Un { i: 0 };
    let ui_offset = 0usize;
    let ud_offset = 0usize;
    if ui_offset != ud_offset {
        exit(7);
    }

    exit(0);
}