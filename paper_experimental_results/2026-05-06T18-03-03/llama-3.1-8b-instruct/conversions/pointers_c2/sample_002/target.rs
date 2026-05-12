fn main() {
    let x: i32 = 17;
    let p: *const i32 = &x;

    let pc: *const i32 = p;
    let pv: *const i32 = p;
    let pcv: *const i32 = p;

    if pc != p {
        return 1;
    }
    if pv != p {
        return 2;
    }
    if pcv != p {
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