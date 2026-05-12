fn main() {
    let x = 0;
    let px1 = &x;
    let px2 = &x;
    if px1 as *const _!= px2 as *const _ {
        return;
    }

    let arr = [1, 2, 3];
    let p_end1 = &arr[3];
    let p_end2 = arr.as_ptr().add(3);
    if p_end1 as *const _!= p_end2 as *const _ {
        return;
    }

    let p0 = &arr[0];
    let p2 = &arr[2];
    if!(*p2 > *p0) {
        return;
    }
    if!(*p0 < *p2) {
        return;
    }

    let q_last = &arr[2];
    let q1 = q_last as *const _ as *const i32;
    if!(*q1 > *p0) {
        return;
    }

    let mut s = std::mem::MaybeUninit::<(i32, i32)>::uninit();
    let sa = s.as_mut_ptr().cast::<u8>();
    let sb = sa.add(4);
    if!(*sb > *sa) {
        return;
    }

    let mut u = std::mem::MaybeUninit::<std::mem::Discriminant>::uninit();
    let ui = u.as_mut_ptr().cast::<u8>();
    let ud = ui;
    if ui!= ud {
        return;
    }

    std::process::exit(0);
}