fn main() {
    let np1: *const i32 = std::ptr::null();
    let np2: *const i32 = std::ptr::null();
    if np1!= np2 {
        return 1;
    }

    let s = (0, 0);
    let ps: *const _ = &s;
    let px: *const _ = &s.0;
    if ps!= px {
        return 2;
    }

    let a = [1, 2, 3];
    let end1: *const i32 = &a[3];
    let end2: *const i32 = a.as_ptr().add(3);
    if end1!= end2 {
        return 3;
    }

    return 0;
}