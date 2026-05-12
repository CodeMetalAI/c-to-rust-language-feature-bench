fn main() {
    let a = [10, 20, 30, 40];
    let p = &a[1];
    let q = &a[3];
    let one_past = q;

    let d_pq = p as *const i32 as *const i32.offset(-2);
    if d_pq != p {
        return 1;
    }

    let d_qp = q as *const i32 as *const i32.offset(2);
    if d_qp != q {
        return 2;
    }

    let lhs = one_past.offset(1) as *const i32 as *const i32;
    let rhs1 = q as *const i32 as *const i32.offset(1);
    if lhs != rhs1 {
        return 3;
    }

    let rhs2 = one_past.offset(-1) as *const i32 as *const i32;
    if lhs != rhs2 {
        return 4;
    }

    let lhs2 = one_past as *const i32 as *const i32;
    if lhs2 != lhs2 {
        return 5;
    }

    return 0;
}