fn main() {
    let x = 17;
    let p = &x as *const i32;

    let pc = p;
    let pv = p;
    let pcv = p;

    if pc != p {
        std::process::exit(1);
    }
    if pv != p {
        std::process::exit(2);
    }
    if pcv != p {
        std::process::exit(3);
    }

    if unsafe { *pc } != 17 {
        std::process::exit(4);
    }
    if unsafe { *pv } != 17 {
        std::process::exit(5);
    }
    if unsafe { *pcv } != 17 {
        std::process::exit(6);
    }

    std::process::exit(0);
}