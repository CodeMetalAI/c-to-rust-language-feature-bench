fn main() -> i32 {
    let x = 17;
    let p = &x;
    let pc = p;
    let pv = p;
    let pcv = p;

    if *pc != 17 {
        return 4;
    }
    if *pv != 17 {
        return 5;
    }
    if *pcv != 17 {
        return 6;
    }

    0
}