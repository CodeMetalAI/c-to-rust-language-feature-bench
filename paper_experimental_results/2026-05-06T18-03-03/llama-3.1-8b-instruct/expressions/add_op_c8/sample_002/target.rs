fn main() {
    let a = [10, 20, 30, 40, 50];

    let p0 = &a[0];
    let p2 = p0.offset(2);
    let p4 = p2.offset(2);
    let p_last = &a[4];
    let p_end = p_last.offset(1);

    if *p2 != 30 {
        return 1;
    }
    if *p4 != 50 {
        return 2;
    }

    let p_back = p4.offset(-3);
    if *p_back != 20 {
        return 3;
    }

    let p_from_end = p_end.offset(-1);
    if p_from_end != p_last {
        return 4;
    }
    if *p_from_end != 50 {
        return 5;
    }

    if (p0.offset(5)) != p_end {
        return 6;
    }

    return 0;
}