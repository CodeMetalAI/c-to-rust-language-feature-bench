fn set_g_return(gval: i32, ret: bool) -> bool {
    static mut G: i32 = 0;
    unsafe {
        G = gval;
    }
    ret
}

fn main() {
    let mut x: i32;

    unsafe {
        G = 0;
    }
    x = if set_g_return(1, true) {
        if unsafe { G } == 1 {
            1
        } else {
            set_g_return(2, true);
            0
        }
    } else {
        0
    };
    if x!= 1 {
        std::process::exit(1);
    }
    if unsafe { G }!= 1 {
        std::process::exit(2);
    }

    unsafe {
        G = 0;
    }
    x = if set_g_return(1, false) {
        set_g_return(2, true);
        0
    } else {
        if unsafe { G } == 1 {
            1
        } else {
            0
        }
    };
    if x!= 1 {
        std::process::exit(3);
    }
    if unsafe { G }!= 1 {
        std::process::exit(4);
    }

    std::process::exit(0);
}