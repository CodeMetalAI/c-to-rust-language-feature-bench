fn main() {
    let a = [10, 20, 30, 40];

    let p = &a[1];
    let q = &a[3];
    let one_past = q;

    let d_pq = p as *const _ as isize - q as *const _ as isize;
    assert_eq!(d_pq, -2);

    let d_qp = q as *const _ as isize - p as *const _ as isize;
    assert_eq!(d_qp, 2);

    let lhs = (one_past as *const _ as isize - p as *const _ as isize);
    let rhs1 = (q as *const _ as isize - p as *const _ as isize) + 1;
    assert_eq!(lhs, rhs1);

    let rhs2 = -(p as *const _ as isize - one_past as *const _ as isize);
    assert_eq!(lhs, rhs2);

    let one_past_ptr = one_past as *const _ as isize;
    assert_eq!(one_past_ptr - one_past_ptr, 0);

    assert!(0 == 0);
}