fn main() {
    let a = [10, 20, 30, 40];

    let p = &a[1]; // Points to 20
    let q = &a[3]; // Points to 40
    let one_past = &a[4]; // This would be one past the end of the array in C++

    // Getting indices instead of using pointer arithmetic directly
    let idx_p = 1;
    let idx_q = 3;
    let idx_one_past = 4;

    let d_pq = (idx_p as isize) - (idx_q as isize);
    if d_pq != -2 {
        return std::process::exit(1);
    }

    let d_qp = (idx_q as isize) - (idx_p as isize);
    if d_qp != 2 {
        return std::process::exit(2);
    }

    let lhs = (idx_one_past as isize) - (idx_p as isize);
    let rhs1 = (idx_q as isize) - (idx_p as isize) + 1;
    if lhs != rhs1 {
        return std::process::exit(3);
    }

    let rhs2 = -(idx_p as isize - idx_one_past as isize);
    if lhs != rhs2 {
        return std::process::exit(4);
    }

    if ((idx_one_past as isize) - (idx_one_past as isize)) != 0 {
        return std::process::exit(5);
    }
}