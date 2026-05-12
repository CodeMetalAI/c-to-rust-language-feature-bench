fn main() -> i32 {
    let x = 17;
    let p = &x as *const i32;

    let pc = p as *const i32;
    let pv = p as *const i32;
    let pcv = p as *const i32;

    if pc != p {
        return 1;
    }
    if pv != p {
        return 2;
    }
    if pcv != p {
        return 3;
    }

    unsafe {
        if *pc != 17 {
            return 4;
        }
        if *pv != 17 {
            return 5;
        }
        if *pcv != 17 {
            return 6;
        }
    }

    return 0;
}