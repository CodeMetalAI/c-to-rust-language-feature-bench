fn main() {
    let mut x = 17;
    let p: *const i32 = &x;

    let pc: *const i32 = p;
    let pv: *mut i32 = &mut x as *mut i32;
    let pcv: *const i32 = p;

    if pc != p {
        std::process::exit(1);
    }
    if pv as *const i32 != p {
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