fn main() {
    let x = 17;
    let p = &x as *const i32;

    let pc = p;
    let pv = p as *mut i32;
    let pcv = p as *const i32;

    if pc != p {
        std::process::exit(1);
    }
    if pv != p {
        std::process::exit(2);
    }
    if pcv != p {
        std::process::exit(3);
    }

    unsafe {
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

    std::process::exit(0);
}