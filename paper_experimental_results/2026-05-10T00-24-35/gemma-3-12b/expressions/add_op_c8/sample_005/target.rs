fn main() {
    let a = [10, 20, 30, 40, 50];

    let p0 = &a[0];
    let p2 = &a[(p0 as *const i32).offset(6) as *const i32];
    let p4 = &a[(p2 as *const i32).offset(6) as *const i32];
    let p_last = &a[4];
    let p_end = &a[5];

    if *p2 != 30 {
        return 1;
    }
    if *p4 != 50 {
        return 2;
    }

    let p_back = &a[(p4 as *const i32).offset(-6) as *const i32];
    if *p_back != 20 {
        return 3;
    }

    let p_from_end = &a[4];
    if p_from_end != p_last {
        return 4;
    }
    if *p_from_end != 50 {
        return 5;
    }

    if (&a[5] as *const i32) != p_end {
        return 6;
    }

    0
}