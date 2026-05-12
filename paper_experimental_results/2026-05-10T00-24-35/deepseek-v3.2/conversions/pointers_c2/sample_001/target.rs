fn main() {
    let x: i32 = 17;
    let p: &i32 = &x;

    let pc: &i32 = p;
    let pv: &i32 = p;
    let pcv: &i32 = p;

    if (pc as *const i32) != (p as *const i32) {
        return println!("1");
    }
    if (pv as *const i32) != (p as *const i32) {
        return println!("2");
    }
    if (pcv as *const i32) != (p as *const i32) {
        return println!("3");
    }

    if *pc != 17 {
        return println!("4");
    }
    if *pv != 17 {
        return println!("5");
    }
    if *pcv != 17 {
        return println!("6");
    }

    println!("0");
}