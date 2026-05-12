fn main() -> i32 {
    let a: [i32; 4] = [10, 20, 30, 40];

    let p_index: usize = 1;
    let q_index: usize = 3;
    let one_past_index: usize = q_index + 1;

    let d_pq: isize = p_index as isize - q_index as isize;
    if d_pq != -2 {
        return 1;
    }

    let d_qp: isize = q_index as isize - p_index as isize;
    if d_qp != 2 {
        return 2;
    }

    let lhs: isize = one_past_index as isize - p_index as isize;
    let rhs1: isize = (q_index as isize - p_index as isize) + 1;
    if lhs != rhs1 {
        return 3;
    }

    let rhs2: isize = -(p_index as isize - one_past_index as isize);
    if lhs != rhs2 {
        return 4;
    }

    if (one_past_index as isize - one_past_index as isize) != 0 {
        return 5;
    }

    0
}