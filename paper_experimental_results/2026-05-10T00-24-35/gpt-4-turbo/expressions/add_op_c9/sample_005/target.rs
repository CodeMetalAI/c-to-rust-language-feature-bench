fn main() {
    let a = [10, 20, 30, 40];
    
    let p = &a[1];
    let q = &a[3];
    let one_past = &a[4]; // equivalent to one past the end of the element referenced by q

    let d_pq = (p as *const i32 as isize) - (q as *const i32 as isize);
    if d_pq != -2 {
        return std::process::exit(1);
    }

    let d_qp = (q as *const i32 as isize) - (p as *const i32 as isize);
    if d_qp != 2 {
        return std::process::exit(2);
    }

    let lhs = (one_past as *const i32 as isize) - (p as *const i32 as isize);
    let rhs1 = ((q as *const i32 as isize) - (p as *const i32 as isize)) + 1;
    if lhs != rhs1 {
        return std::process::exit(3);
    }

    let rhs2 = -((p as *const i32 as isize) - (one_past as *const i32 as isize));
    if lhs != rhs2 {
        return std::process::exit(4);
    }

    if ((one_past as *const i32 as isize) - (one_past as *const i32 as isize)) != 0 {
        return std::process::exit(5);
    }
}