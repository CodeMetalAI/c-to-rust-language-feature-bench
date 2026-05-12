fn main() {
    let x = 0;
    let px1 = &x as *const i32;
    let px2 = &x as *const i32;
    if px1!= px2 {
        return;
    }

    let arr = [1, 2, 3];
    let p_end1 = &arr[2] as *const i32;
    let p_end2 = arr.as_ptr().add(3);
    if p_end1!= p_end2 {
        return;
    }

    let p0 = &arr[0] as *const i32;
    let p2 = &arr[2] as *const i32;
    if!(p2 > p0) {
        return;
    }
    if!(p0 < p2) {
        return;
    }

    let q_last = &arr[2] as *const i32;
    let q1 = q_last.add(1);
    if!(q1 > p0) {
        return;
    }

    let s = [0, 0];
    let sa = s.as_ptr() as *const u8;
    let sb = s.as_ptr().add(1) as *const u8;
    if!(sb > sa) {
        return;
    }

    let u = [0; std::mem::size_of::<i32>()];
    let ui = u.as_ptr() as *const u8;
    let ud = u.as_ptr() as *const u8;
    if ui!= ud {
        return;
    }

    std::process::exit(0);
}