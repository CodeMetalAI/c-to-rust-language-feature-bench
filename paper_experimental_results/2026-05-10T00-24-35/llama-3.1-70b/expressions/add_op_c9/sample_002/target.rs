fn main() {
    let a: [i32; 4] = [10, 20, 30, 40];

    let p = &a[1];
    let q = &a[3];
    let one_past = unsafe { a.as_ptr().offset(4) };

    let d_pq: isize = (p as *const i32 as isize) - (q as *const i32 as isize);
    if d_pq!= -2 {
        std::process::exit(1);
    }

    let d_qp: isize = (q as *const i32 as isize) - (p as *const i32 as isize);
    if d_qp!= 2 {
        std::process::exit(2);
    }

    let lhs: isize = (one_past as *const i32 as isize) - (p as *const i32 as isize);
    let rhs1: isize = (q as *const i32 as isize) - (p as *const i32 as isize) + 1;
    if lhs!= rhs1 {
        std::process::exit(3);
    }

    let rhs2: isize = -((p as *const i32 as isize) - (one_past as *const i32 as isize));
    if lhs!= rhs2 {
        std::process::exit(4);
    }

    if (one_past as *const i32 as isize) - (one_past as *const i32 as isize)!= 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}