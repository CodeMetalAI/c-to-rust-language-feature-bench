static mut G: i32 = 0;

fn set_g_return(gval: i32, ret: bool) -> bool {
    unsafe {
        G = gval;
    }
    ret
}

fn main() {
    let x;
    unsafe {
        G = 0;
    }
    x = if set_g_return(1, true) {
        unsafe { G } == 1
    } else {
        set_g_return(2, true);
        false
    };
    if x!= true {
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
        false
    } else {
        unsafe { G } == 1
    };
    if x!= true {
        std::process::exit(3);
    }
    if unsafe { G }!= 1 {
        std::process::exit(4);
    }
}