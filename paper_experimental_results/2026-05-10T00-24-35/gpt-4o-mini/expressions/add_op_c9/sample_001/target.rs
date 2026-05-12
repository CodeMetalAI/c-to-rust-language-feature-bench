fn main() -> i32 {
    let a = [10, 20, 30, 40];

    let p = &a[1];
    let q = &a[3];
    let one_past = unsafe { &a[4] }; // Safe access to one past the end of the array

    let d_pq = (p as *const _ as usize - q as *const _ as usize) / std::mem::size_of::<i32>();
    if d_pq != -2 {
        return 1;
    }

    let d_qp = (q as *const _ as usize - p as *const _ as usize) / std::mem::size_of::<i32>();
    if d_qp != 2 {
        return 2;
    }

    let lhs = (one_past as *const _ as usize - p as *const _ as usize) / std::mem::size_of::<i32>();
    let rhs1 = (q as *const _ as usize - p as *const _ as usize) / std::mem::size_of::<i32>() + 1;
    if lhs != rhs1 {
        return 3;
    }

    let rhs2 = -(p as *const _ as usize - one_past as *const _ as usize) / std::mem::size_of::<i32>();
    if lhs != rhs2 {
        return 4;
    }

    if (one_past as *const _ as usize - one_past as *const _ as usize) / std::mem::size_of::<i32>() != 0 {
        return 5;
    }

    0
}