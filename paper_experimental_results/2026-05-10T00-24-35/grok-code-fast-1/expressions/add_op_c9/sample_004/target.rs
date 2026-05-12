fn main() {
    let a = [10, 20, 30, 40];

    let p_index = 1;
    let q_index = 3;
    let one_past_index = q_index + 1;

    let d_pq = p_index as isize - q_index as isize;
    if d_pq != -2 {
        std::process::exit(1);
    }

    let d_qp = q_index as isize - p_index as isize;
    if d_qp != 2 {
        std::process::exit(2);
    }

    let lhs = one_past_index as isize - p_index as isize;
    let rhs1 = (q_index as isize - p_index as isize) + 1;
    if lhs != rhs1 {
        std::process::exit(3);
    }

    let rhs2 = -(p_index as isize - one_past_index as isize);
    if lhs != rhs2 {
        std::process::exit(4);
    }

    if (one_past_index as isize - one_past_index as isize) != 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}