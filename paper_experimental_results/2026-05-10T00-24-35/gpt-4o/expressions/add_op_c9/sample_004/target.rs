fn main() {
    let a = [10, 20, 30, 40];

    let p = &a[1] as *const i32;
    let q = &a[3] as *const i32;
    let one_past = unsafe { q.add(1) };

    let d_pq = p.wrapping_offset_from(q);
    if d_pq != -2 {
        std::process::exit(1);
    }

    let d_qp = q.wrapping_offset_from(p);
    if d_qp != 2 {
        std::process::exit(2);
    }

    let lhs = one_past.wrapping_offset_from(p);
    let rhs1 = q.wrapping_offset_from(p) + 1;
    if lhs != rhs1 {
        std::process::exit(3);
    }

    let rhs2 = -(p.wrapping_offset_from(one_past));
    if lhs != rhs2 {
        std::process::exit(4);
    }

    if one_past.wrapping_offset_from(one_past) != 0 {
        std::process::exit(5);
    }
}