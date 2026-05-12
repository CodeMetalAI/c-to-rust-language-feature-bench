fn main() {
    let mut a = [10, 20, 30, 40];

    let p = &a[1];
    let q = &a[3];
    let one_past = &a[3];

    let d_pq = p as *const _ as usize - q as *const _ as usize;
    if d_pq!= -2 {
        return;
    }

    let d_qp = q as *const _ as usize - p as *const _ as usize;
    if d_qp!= 2 {
        return;
    }

    let lhs = (one_past as *const _ as usize) - (p as *const _ as usize);
    let rhs1 = (q as *const _ as usize) - (p as *const _ as usize) + 1;
    if lhs!= rhs1 {
        return;
    }

    let rhs2 = -(p as *const _ as usize) - (one_past as *const _ as usize);
    if lhs!= rhs2 {
        return;
    }

    if (one_past as *const _ as usize) - (one_past as *const _ as usize)!= 0 {
        return;
    }

    println!("0");
}