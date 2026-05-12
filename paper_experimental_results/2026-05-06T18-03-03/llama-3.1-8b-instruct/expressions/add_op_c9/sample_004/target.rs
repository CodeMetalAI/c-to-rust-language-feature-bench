fn main() {
    let a = [10, 20, 30, 40];

    let p = &a[1];
    let q = &a[3];
    let one_past = q;

    let d_pq = p as *const _ as usize - q as *const _ as usize;
    if d_pq != -2 {
        return 1;
    }

    let d_qp = q as *const _ as usize - p as *const _ as usize;
    if d_qp != 2 {
        return 2;
    }

    let lhs = one_past as *const _ as usize - p as *const _ as usize;
    let rhs1 = q as *const _ as usize - p as *const _ as usize + 1;
    if lhs != rhs1 {
        return 3;
    }

    let rhs2 = -(p as *const _ as usize - one_past as *const _ as usize);
    if lhs != rhs2 {
        return 4;
    }

    if one_past as *const _ as usize - one_past as *const _ as usize != 0 {
        return 5;
    }

    return 0;
}