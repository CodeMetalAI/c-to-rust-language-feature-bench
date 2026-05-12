fn main() {
    let a = [10, 20, 30, 40];

    let p = &a[1];
    let q = &a[3];
    let one_past = &a[4]; // one past the last element

    let d_pq = (p as *const _ as usize) as isize - (q as *const _ as usize) as isize;
    if d_pq != -2 {
        std::process::exit(1);
    }

    let d_qp = (q as *const _ as usize) as isize - (p as *const _ as usize) as isize;
    if d_qp != 2 {
        std::process::exit(2);
    }

    let lhs = (one_past as *const _ as usize) as isize - (p as *const _ as usize) as isize;
    let rhs1 = (q as *const _ as usize) as isize - (p as *const _ as usize) as isize + 1;
    if lhs != rhs1 {
        std::process::exit(3);
    }

    let rhs2 = -(p as *const _ as usize as isize - one_past as *const _ as usize as isize);
    if lhs != rhs2 {
        std::process::exit(4);
    }

    if (one_past as *const _ as usize as isize - one_past as *const _ as usize as isize) != 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}