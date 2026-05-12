fn main() {
    let a = [10, 20, 30, 40];

    let p = &a[1];
    let q = &a[3];
    let one_past = &a[4]; // Safe because a is of fixed size

    let d_pq = (p as *const i32).wrapping_sub((q as *const i32)) as ptrdiff_t;
    if d_pq != -2 {
        std::process::exit(1);
    }

    let d_qp = (q as *const i32).wrapping_sub((p as *const i32)) as ptrdiff_t;
    if d_qp != 2 {
        std::process::exit(2);
    }

    let lhs = ((one_past as *const i32).wrapping_sub((p as *const i32)) as ptrdiff_t);
    let rhs1 = ((q as *const i32).wrapping_sub((p as *const i32)) as ptrdiff_t) + 1;
    if lhs != rhs1 {
        std::process::exit(3);
    }

    let rhs2 = -((p as *const i32).wrapping_sub((one_past as *const i32)) as ptrdiff_t);
    if lhs != rhs2 {
        std::process::exit(4);
    }

    if ((one_past as *const i32).wrapping_sub((one_past as *const i32)) as ptrdiff_t) != 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}