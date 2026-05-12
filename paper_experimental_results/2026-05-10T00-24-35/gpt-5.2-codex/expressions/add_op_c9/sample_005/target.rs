use std::process::exit;

fn main() {
    let a = [10, 20, 30, 40];

    let p: isize = 1;
    let q: isize = 3;
    let one_past: isize = q + 1;

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

    // Use the array to avoid unused warning (though not necessary for correctness)
    let _ = a;
}