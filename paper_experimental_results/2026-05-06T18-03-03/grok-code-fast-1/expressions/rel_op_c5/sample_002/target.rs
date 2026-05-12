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
    // Simulate addresses using usize
    let addr_x: usize = 1000;
    let px1 = addr_x;
    let px2 = addr_x;
    if px1 != px2 {
        std::process::exit(1);
    }

    let addr_arr: usize = 2000;
    let elem_size = std::mem::size_of::<i32>() as usize;
    let p_end1 = addr_arr + 3 * elem_size;
    let p_end2 = addr_arr + 3 * elem_size;
    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    let p0 = addr_arr;
    let p2 = addr_arr + 2 * elem_size;
    if !(p2 > p0) {
        std::process::exit(3);
    }
    if !(p0 < p2) {
        std::process::exit(4);
    }

    let q_last = addr_arr + 2 * elem_size;
    let q1 = q_last + elem_size;
    if !(q1 > p0) {
        std::process::exit(5);
    }

    let addr_s: usize = 3000;
    let sa = addr_s;
    let sb = addr_s + std::mem::size_of::<i32>() as usize;
    if !(sb > sa) {
        std::process::exit(6);
    }

    let addr_u: usize = 4000;
    let ui = addr_u;
    let ud = addr_u;
    if ui != ud {
        std::process::exit(7);
    }

    std::process::exit(0);
}