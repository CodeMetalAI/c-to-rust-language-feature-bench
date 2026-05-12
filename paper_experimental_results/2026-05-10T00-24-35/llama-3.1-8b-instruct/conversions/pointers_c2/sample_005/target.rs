fn main() {
    let x = 17;
    let mut p: *const i32 = &x;
    let pc: *const i32 = &x;
    let pv: *mut i32 = &mut x;
    let cpv: *const mut i32 = &mut x;

    if pc != p {
        return 1;
    }
    if pv != p {
        return 2;
    }
    if cpv != p {
        return 3;
    }

    if unsafe { *pc } != 17 {
        return 4;
    }
    if unsafe { *pv } != 17 {
        return 5;
    }
    if unsafe { *cpv } != 17 {
        return 6;
    }

    return 0;
}