fn main() {
    let a: [i32; 4] = [10, 20, 30, 40];

    let p_idx: isize = 1;
    let q_idx: isize = 3;
    let one_past_idx: isize = q_idx + 1;

    let d_pq = p_idx - q_idx;
    if d_pq != -2 {
        std::process::exit(1);
    }

    let d_qp = q_idx - p_idx;
    if d_qp != 2 {
        std::process::exit(2);
    }

    let lhs = one_past_idx - p_idx;
    let rhs1 = (q_idx - p_idx) + 1;
    if lhs != rhs1 {
        std::process::exit(3);
    }

    let rhs2 = -(p_idx - one_past_idx);
    if lhs != rhs2 {
        std::process::exit(4);
    }

    if (one_past_idx - one_past_idx) != 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}