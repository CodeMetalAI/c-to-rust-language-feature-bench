fn main() -> i32 {
    let _a: [i32; 4] = [10, 20, 30, 40];

    let p_idx: isize = 1;
    let q_idx: isize = 3;
    let one_past_idx: isize = 4;

    let d_pq: isize = p_idx - q_idx;
    if d_pq != -2 {
        return 1;
    }

    let d_qp: isize = q_idx - p_idx;
    if d_qp != 2 {
        return 2;
    }

    let lhs: isize = one_past_idx - p_idx;
    let rhs1: isize = (q_idx - p_idx) + 1;
    if lhs != rhs1 {
        return 3;
    }

    let rhs2: isize = -(p_idx - one_past_idx);
    if lhs != rhs2 {
        return 4;
    }

    if (one_past_idx - one_past_idx) != 0 {
        return 5;
    }

    0
}