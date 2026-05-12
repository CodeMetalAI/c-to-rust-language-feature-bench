fn main() {
    let a = [10, 20, 30, 40];

    let p = &a[1];
    let q = &a[3];
    let one_past = &a[3];

    let d_pq = p.offset(-2);
    if d_pq != q {
        return 1;
    }

    let d_qp = p.offset(2);
    if d_qp != q {
        return 2;
    }

    let lhs = one_past.offset(-1);
    let rhs1 = q.offset(1);
    if lhs != rhs1 {
        return 3;
    }

    let rhs2 = p.offset(-1);
    if lhs != rhs2 {
        return 4;
    }

    if one_past.offset(0) != one_past {
        return 5;
    }

    0
}