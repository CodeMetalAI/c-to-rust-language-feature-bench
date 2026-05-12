fn main() {
    let a = [10, 20, 30, 40, 50];

    let p0 = a.as_ptr();
    let p2 = a.as_ptr().offset(2);
    let p4 = p2.offset(2);
    let p_last = a.as_ptr().offset(4);
    let p_end = p_last.offset(1);

    if unsafe { *p2 } != 30 {
        return;
    }
    if unsafe { *p4 } != 50 {
        return;
    }

    let p_back = p4.offset(-3);
    if unsafe { *p_back } != 20 {
        return;
    }

    let p_from_end = p_end.offset(-1);
    if p_from_end != p_last {
        return;
    }
    if unsafe { *p_from_end } != 50 {
        return;
    }

    if p0.offset(5) != p_end {
        return;
    }
}