fn main() {
    let a = [10, 20, 30, 40];

    let p = &a[1];
    let q = &a[3];
    let one_past = &a[4];

    let d_pq = (p as *const i32 as isize) - (q as *const i32 as isize);
    if d_pq != -2 {
        return;
    }

    let d_qp = (q as *const i32 as isize) - (p as *const i32 as isize);
    if d_qp != 2 {
        return;
    }

    let lhs = (one_past as *const i32 as isize) - (p as *const i32 as isize);
    let rhs1 = (q as *const i32 as isize) - (p as *const i32 as isize) + 1;
    if lhs != rhs1 {
        return;
    }

    let rhs2 = -(lhs);
    if lhs != rhs2 {
        return;
    }

    if (one_past as *const i32 as isize) - (one_past as *const i32 as isize) != 0 {
        return;
    }

    println!("");
}