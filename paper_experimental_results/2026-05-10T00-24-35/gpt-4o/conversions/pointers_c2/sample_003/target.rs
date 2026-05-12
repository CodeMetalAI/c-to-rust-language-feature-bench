fn main() {
    let x = 17;
    let p = &x;

    let pc: &i32 = p;
    let pv: &i32 = p;
    let pcv: &i32 = p;

    if (pc as *const i32 as *const ()) != (p as *const i32 as *const ()) {
        std::process::exit(1);
    }
    if (pv as *const i32 as *const ()) != (p as *const i32 as *const ()) {
        std::process::exit(2);
    }
    if (pcv as *const i32 as *const ()) != (p as *const i32 as *const ()) {
        std::process::exit(3);
    }

    if *pc != 17 {
        std::process::exit(4);
    }
    if *pv != 17 {
        std::process::exit(5);
    }
    if *pcv != 17 {
        std::process::exit(6);
    }
}