fn main() {
    let a = [10, 20, 30, 40];

    let p = &a[1];
    let q = &a[3];
    let one_past = q as *const i32 + 1;

    let d_pq = (p as *const i32).offset_from(q as *const i32);
    if d_pq != -2 {
        std::process::exit(1);
    }

    let d_qp = (q as *const i32).offset_from(p as *const i32);
    if d_qp != 2 {
        std::process::exit(2);
    }

    let lhs = (one_past).offset_from(p as *const i32);
    let rhs1 = (q as *const i32).offset_from(p as *const i32) + 1;
    if lhs != rhs1 {
        std::process::exit(3);
    }

    let rhs2 = -((p as *const i32).offset_from(one_past));
    if lhs != rhs2 {
        std::process::exit(4);
    }

    if (one_past).offset_from(one_past) != 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}