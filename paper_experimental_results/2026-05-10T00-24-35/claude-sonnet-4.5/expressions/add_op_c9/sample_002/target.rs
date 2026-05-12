fn main() {
    let a: [i32; 4] = [10, 20, 30, 40];

    let p_idx = 1;
    let q_idx = 3;
    let one_past_idx = q_idx + 1;

    let d_pq = (p_idx as isize) - (q_idx as isize);
    if d_pq != -2 {
        std::process::exit(1);
    }

    let d_qp = (q_idx as isize) - (p_idx as isize);
    if d_qp != 2 {
        std::process::exit(2);
    }

    let lhs = (one_past_idx as isize) - (p_idx as isize);
    let rhs1 = ((q_idx as isize) - (p_idx as isize)) + 1;
    if lhs != rhs1 {
        std::process::exit(3);
    }

    let rhs2 = -((p_idx as isize) - (one_past_idx as isize));
    if lhs != rhs2 {
        std::process::exit(4);
    }

    if ((one_past_idx as isize) - (one_past_idx as isize)) != 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}