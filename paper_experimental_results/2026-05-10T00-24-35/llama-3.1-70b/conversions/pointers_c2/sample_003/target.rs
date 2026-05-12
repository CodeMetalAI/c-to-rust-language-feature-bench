fn main() {
    let x: i32 = 17;
    let p: &i32 = &x;

    let pc: &i32 = p;
    let pv: &i32 = p;
    let pcv: &i32 = p;

    if (pc as *const i32 as *const ())!= (p as *const i32 as *const ())
        return 1;
    if (pv as *const i32 as *const ())!= (p as *const i32 as *const ())
        return 2;
    if (pcv as *const i32 as *const ())!= (p as *const i32 as *const ())
        return 3;

    if *pc!= 17
        return 4;
    if *pv!= 17
        return 5;
    if *pcv!= 17
        return 6;

    return 0;
}