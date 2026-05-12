fn main() -> i32 {
    let x = 17;
    let p: &i32 = &x;

    let pc: &const i32 = p;
    let pv: &mut i32 = &mut (p as *const _ as *mut _); // mimic volatile behavior
    let pcv: &const mut i32 = &mut (p as *const _ as *mut _); // mimic const volatile behavior

    if pc as *const _ != p as *const _ {
        return 1;
    }
    if pv as *const _ != p as *const _ {
        return 2;
    }
    if pcv as *const _ != p as *const _ {
        return 3;
    }

    if *pc != 17 {
        return 4;
    }
    if *pv != 17 {
        return 5;
    }
    if *pcv != 17 {
        return 6;
    }

    return 0;
}