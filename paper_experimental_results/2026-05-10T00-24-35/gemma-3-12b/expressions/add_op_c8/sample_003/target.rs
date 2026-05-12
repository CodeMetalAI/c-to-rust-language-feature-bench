fn main() {
    let a = [10, 20, 30, 40, 50];

    let p0 = &a[0];
    let p2 = &a[2];
    let p4 = &a[4];
    let p_last = &a[4];
    let p_end = &a[5];

    if *p2 != 30 {
        return 1;
    }
    if *p4 != 50 {
        return 2;
    }

    let p_back = &a[1];
    if *p_back != 20 {
        return 3;
    }

    if p_from_end(&a, p_end) != p_last {
        return 4;
    }
    if *p_from_end(&a, p_end) != 50 {
        return 5;
    }

    if p0_plus_5(&a, p0) != p_end {
        return 6;
    }

    0
}

fn p_from_end(a: &[i32], p_end: &i32) -> &i32 {
    let ptr_to_i32_ref = p_end as *const i32;
    let ptr_to_i32_ref_minus_one = ptr_to_i32_ref.offset(-1);
    unsafe { &*ptr_to_i32_ref_minus_one }
}

fn p0_plus_5(a: &[i32], p0: &i32) -> &i32 {
    let ptr_to_i32_ref = p0 as *const i32;
    let ptr_to_i32_ref_plus_5 = ptr_to_i32_ref.offset(5);
    unsafe { &*ptr_to_i32_ref_plus_5 }
}