fn main() {
    let a: [i32; 4] = [10, 20, 30, 40];

    let p: usize = 1;
    let q: usize = 3;
    let one_past: usize = q + 1;

    let d_pq: isize = p as isize - q as isize;
    if d_pq != -2 {
        std::process::exit(1);
    }

    let d_qp: isize = q as isize - p as isize;
    if d_qp != 2 {
        std::process::exit(2);
    }

    let lhs: isize = one_past as isize - p as isize;
    let rhs1: isize = (q as isize - p as isize) + 1;
    if lhs != rhs1 {
        std::process::exit(3);
    }

    let rhs2: isize = -(p as isize - one_past as isize);
    if lhs != rhs2 {
        std::process::exit(4);
    }

    if (one_past as isize - one_past as isize) != 0 {
        std::process::exit(5);
    }

    // Suppress unused variable warning
    let _ = a;

    std::process::exit(0);
}