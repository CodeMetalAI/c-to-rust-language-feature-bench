#[derive(Debug)]
struct St {
    a: i32,
    b: i32,
}

#[derive(Debug)]
union Un {
    i: i32,
    d: f64,
}

fn main() {
    let x = 0;
    let px1 = &x;
    let px2 = &x;
    if px1 != px2 {
        std::process::exit(1);
    }

    let arr = vec![1, 2, 3];
    let p_end1 = 3;
    let p_end2 = 3;
    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    let p0 = 0;
    let p2 = 2;
    if !(p2 > p0) {
        std::process::exit(3);
    }
    if !(p0 < p2) {
        std::process::exit(4);
    }

    let q_last = 2;
    let q1 = q_last + 1;
    if !(q1 > p0) {
        std::process::exit(5);
    }

    let sa_offset = std::mem::offset_of!(St, a);
    let sb_offset = std::mem::offset_of!(St, b);
    if !(sb_offset > sa_offset) {
        std::process::exit(6);
    }

    let ui_offset = std::mem::offset_of!(Un, i);
    let ud_offset = std::mem::offset_of!(Un, d);
    if ui_offset != ud_offset {
        std::process::exit(7);
    }

    std::process::exit(0);
}