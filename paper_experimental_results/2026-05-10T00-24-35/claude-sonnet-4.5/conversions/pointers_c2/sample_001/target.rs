fn main() {
    let mut x = 17;
    let p = &mut x as *mut i32;

    let pc = p as *const i32;
    let pv = p as *mut i32;
    let pcv = p as *const i32;

    if pc as *const () != p as *const () {
        std::process::exit(1);
    }
    if pv as *const () != p as *const () {
        std::process::exit(2);
    }
    if pcv as *const () != p as *const () {
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