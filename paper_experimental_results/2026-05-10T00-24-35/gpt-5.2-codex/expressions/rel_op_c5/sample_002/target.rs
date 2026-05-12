use std::ptr;

#[repr(C)]
struct St {
    a: i32,
    b: i32,
}

fn main() {
    let x = 0;
    let px1 = &x;
    let px2 = &x;
    if !ptr::eq(px1, px2) {
        std::process::exit(1);
    }

    let arr = [1, 2, 3];
    let p_end1 = arr.len();
    let p_end2 = 3usize;
    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    let p0 = 0usize;
    let p2 = 2usize;
    if !(p2 > p0) {
        std::process::exit(3);
    }
    if !(p0 < p2) {
        std::process::exit(4);
    }

    let q_last = 2usize;
    let q1 = q_last + 1;
    if !(q1 > p0) {
        std::process::exit(5);
    }

    let _s = St { a: 0, b: 0 };
    let sa_offset = 0usize;
    let sb_offset = std::mem::size_of::<i32>();
    if !(sb_offset > sa_offset) {
        std::process::exit(6);
    }

    let ui_offset = 0usize;
    let ud_offset = 0usize;
    if ui_offset != ud_offset {
        std::process::exit(7);
    }

    std::process::exit(0);
}