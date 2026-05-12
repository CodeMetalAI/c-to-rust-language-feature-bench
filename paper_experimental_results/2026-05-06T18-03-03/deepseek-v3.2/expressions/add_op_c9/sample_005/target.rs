use std::ptr;

fn main() {
    let a = [10, 20, 30, 40];

    let p = &a[1] as *const i32;
    let q = &a[3] as *const i32;
    let one_past = unsafe { q.add(1) };

    let d_pq = (p as isize).wrapping_sub(q as isize) / std::mem::size_of::<i32>() as isize;
    if d_pq != -2 {
        std::process::exit(1);
    }

    let d_qp = (q as isize).wrapping_sub(p as isize) / std::mem::size_of::<i32>() as isize;
    if d_qp != 2 {
        std::process::exit(2);
    }

    let lhs = (one_past as isize).wrapping_sub(p as isize) / std::mem::size_of::<i32>() as isize;
    let rhs1 = (q as isize).wrapping_sub(p as isize) / std::mem::size_of::<i32>() as isize + 1;
    if lhs != rhs1 {
        std::process::exit(3);
    }

    let rhs2 = -((p as isize).wrapping_sub(one_past as isize) / std::mem::size_of::<i32>() as isize);
    if lhs != rhs2 {
        std::process::exit(4);
    }

    if (one_past as isize).wrapping_sub(one_past as isize) / std::mem::size_of::<i32>() as isize != 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}