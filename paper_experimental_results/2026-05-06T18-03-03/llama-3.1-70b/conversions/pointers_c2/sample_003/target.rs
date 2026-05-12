fn main() {
    let x: i32 = 17;
    let p: *const i32 = &x;

    let pc: *const i32 = p;
    let pv: *const i32 = p;
    let pcv: *const i32 = p;

    if pc as *const ()!= p as *const () {
        return;
    }
    if pv as *const ()!= p as *const () {
        return;
    }
    if pcv as *const ()!= p as *const () {
        return;
    }

    if unsafe { *pc }!= 17 {
        return;
    }
    if unsafe { *pv }!= 17 {
        return;
    }
    if unsafe { *pcv }!= 17 {
        return;
    }
}