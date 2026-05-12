fn set_g_return(gval: i32, ret: bool) -> bool {
    static mut G: i32 = 0;
    unsafe {
        G = gval;
    }
    ret
}

fn main() {
    let mut x: bool;

    unsafe {
        G = 0;
    }
    x = set_g_return(1, true) && (unsafe { G == 1 }) || set_g_return(2, true);
    if!x {
        std::process::exit(1);
    }
    if unsafe { G }!= 1 {
        std::process::exit(2);
    }

    unsafe {
        G = 0;
    }
    x = set_g_return(1, false) && set_g_return(2, true) || (unsafe { G == 1 });
    if!x {
        std::process::exit(3);
    }
    if unsafe { G }!= 1 {
        std::process::exit(4);
    }
}