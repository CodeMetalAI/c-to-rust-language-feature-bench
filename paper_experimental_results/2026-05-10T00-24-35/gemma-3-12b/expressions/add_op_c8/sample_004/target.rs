fn main() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    let p0 = &a[0];
    let p2 = unsafe { &*(p0 as *const i32 + 2) };
    let p4 = unsafe { &*(p2 as *const i32 + 2) };
    let p_last = &a[4];
    let p_end = unsafe { (p_last as *const i32 + 1) as *const i32 };

    if *p2 != 30 {
        return 1;
    }
    if *p4 != 50 {
        return 2;
    }

    let p_back = unsafe { &*(p4 as *const i32 - 3) };
    if *p_back != 20 {
        return 3;
    }

    let p_from_end = unsafe { (p_end as *const i32 - 1) as *const i32 };
    if p_from_end != p_last {
        return 4;
    }
    if *p_from_end != 50 {
        return 5;
    }

    if (p0 as *const i32 + 5) != p_end {
        return 6;
    }

    0
}