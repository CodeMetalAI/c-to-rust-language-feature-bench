use std::ptr;

fn main() {
    let a = [10, 20, 30, 40];

    let p = &a[1];
    let q = &a[3];
    let one_past = unsafe { q.add(1) };

    let d_pq = ptr::sub_ptr(p, q);
    if d_pq != -2 {
        std::process::exit(1);
    }

    let d_qp = ptr::sub_ptr(q, p);
    if d_qp != 2 {
        std::process::exit(2);
    }

    let lhs = ptr::sub_ptr(one_past, p);
    let rhs1 = ptr::sub_ptr(q, p) + 1;
    if lhs != rhs1 {
        std::process::exit(3);
    }

    let rhs2 = -(ptr::sub_ptr(p, one_past));
    if lhs != rhs2 {
        std::process::exit(4);
    }

    if ptr::sub_ptr(one_past, one_past) != 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}