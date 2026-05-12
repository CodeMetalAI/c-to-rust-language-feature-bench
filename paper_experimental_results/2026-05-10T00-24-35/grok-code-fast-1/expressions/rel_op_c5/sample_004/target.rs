fn main() {
    let x_addr: usize = 0;
    let px1: usize = x_addr;
    let px2: usize = x_addr;
    if px1 != px2 {
        std::process::exit(1);
    }

    let arr_addr: usize = 4;
    let p_end1: usize = arr_addr + 3 * 4;
    let p_end2: usize = arr_addr + 3 * 4;
    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    let p0: usize = arr_addr;
    let p2: usize = arr_addr + 2 * 4;
    if !(p2 > p0) {
        std::process::exit(3);
    }
    if !(p0 < p2) {
        std::process::exit(4);
    }

    let q_last: usize = arr_addr + 2 * 4;
    let q1: usize = q_last + 4;
    if !(q1 > p0) {
        std::process::exit(5);
    }

    let s_addr: usize = 16;
    let sa: usize = s_addr + 0;
    let sb: usize = s_addr + 4;
    if !(sb > sa) {
        std::process::exit(6);
    }

    let u_addr: usize = 24;
    let ui: usize = u_addr + 0;
    let ud: usize = u_addr + 0;
    if ui != ud {
        std::process::exit(7);
    }

    std::process::exit(0);
}