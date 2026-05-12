fn main() {
    let mut x: i32 = 17;
    let p = &x;

    let pc = p;
    let pv = p;
    let c_pv = p;

    if pc as *const _!= p as *const _ {
        return 1;
    }
    if pv as *const _!= p as *const _ {
        return 2;
    }
    if c_pv as *const _!= p as *const _ {
        return 3;
    }

    if *pc!= 17 {
        return 4;
    }
    if *pv!= 17 {
        return 5;
    }
    if *c_pv!= 17 {
        return 6;
    }

    return 0;
}