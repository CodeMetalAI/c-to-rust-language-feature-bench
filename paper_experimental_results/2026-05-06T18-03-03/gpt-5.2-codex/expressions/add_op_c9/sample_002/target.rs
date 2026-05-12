use std::process::exit;

fn main() {
    let a = [10, 20, 30, 40];

    let p = 1isize;
    let q = 3isize;
    let one_past = q + 1;

    let d_pq = p - q;
    if d_pq != -2 {
        exit(1);
    }

    let d_qp = q - p;
    if d_qp != 2 {
        exit(2);
    }

    let lhs = one_past - p;
    let rhs1 = (q - p) + 1;
    if lhs != rhs1 {
        exit(3);
    }

    let rhs2 = -(p - one_past);
    if lhs != rhs2 {
        exit(4);
    }

    if (one_past - one_past) != 0 {
        exit(5);
    }

    // exit 0 implicitly
    let _ = a; // suppress unused warning
}