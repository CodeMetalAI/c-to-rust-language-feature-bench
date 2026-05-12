fn main() {
    let x = 17;
    let p = &x;

    let pc: *const i32 = p;
    let pv: *const i32 = p;
    let pcv: *const i32 = p;

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
}