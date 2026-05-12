fn main() {
    let x: i32 = 17;
    let p: *const i32 = &x as *const i32;

    let pc: *const i32 = p;
    let pv: *const i32 = p;
    let pcv: *const i32 = p;

    if pc as *const ()!= p as *const () {
        return 1;
    }
    if pv as *const ()!= p as *const () {
        return 2;
    }
    if pcv as *const ()!= p as *const () {
        return 3;
    }

    if unsafe { *pc }!= 17 {
        return 4;
    }
    if unsafe { *pv }!= 17 {
        return 5;
    }
    if unsafe { *pcv }!= 17 {
        return 6;
    }

    return 0;
}