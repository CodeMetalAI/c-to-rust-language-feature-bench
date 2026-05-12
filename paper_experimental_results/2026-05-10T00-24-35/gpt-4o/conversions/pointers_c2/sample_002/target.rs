fn main() {
    let x = 17;
    let p: &i32 = &x;

    let pc: &i32 = p;
    let pv: &i32 = p;
    let pcv: &i32 = p;

    if pc as *const _ as *const () != p as *const _ as *const () {
        std::process::exit(1);
    }
    if pv as *const _ as *const () != p as *const _ as *const () {
        std::process::exit(2);
    }
    if pcv as *const _ as *const () != p as *const _ as *const () {
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

    std::process::exit(0);
}