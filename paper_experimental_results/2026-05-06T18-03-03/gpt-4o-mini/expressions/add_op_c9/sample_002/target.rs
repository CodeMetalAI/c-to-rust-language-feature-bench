fn main() -> i32 {
    let a = [10, 20, 30, 40];

    let p = &a[1];
    let q = &a[3];
    let one_past = unsafe { &a[4] }; // one past the last element

    let d_pq = (p as *const _ as isize - q as *const _ as isize) / std::mem::size_of::<i32>() as isize;
    if d_pq != -2 {
        return 1;
    }

    let d_qp = (q as *const _ as isize - p as *const _ as isize) / std::mem::size_of::<i32>() as isize;
    if d_qp != 2 {
        return 2;
    }

    let lhs = (one_past as *const _ as isize - p as *const _ as isize) / std::mem::size_of::<i32>() as isize;
    let rhs1 = (q as *const _ as isize - p as *const _ as isize) / std::mem::size_of::<i32>() as isize + 1;
    if lhs != rhs1 {
        return 3;
    }

    let rhs2 = -(p as *const _ as isize - one_past as *const _ as isize) / std::mem::size_of::<i32>() as isize;
    if lhs != rhs2 {
        return 4;
    }

    if (one_past as *const _ as isize - one_past as *const _ as isize) != 0 {
        return 5;
    }

    0
}