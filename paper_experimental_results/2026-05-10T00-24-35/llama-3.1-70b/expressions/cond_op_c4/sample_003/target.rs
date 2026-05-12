fn set_g_return(gval: i32, ret: i32) -> i32 {
    static mut G: i32 = 0;
    unsafe {
        G = gval;
    }
    ret
}

fn main() {
    static mut G: i32 = 0;
    let mut x: i32;

    unsafe {
        G = 0;
    }
    x = if unsafe { set_g_return(1, 1) }!= 0 {
        if unsafe { G } == 1 { 1 } else { 0 }
    } else {
        unsafe {
            set_g_return(2, 1);
        }
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
    x = if unsafe { set_g_return(1, 0) }!= 0 {
        unsafe {
            set_g_return(2, 1);
        }
        0
    } else {
        if unsafe { G } == 1 { 1 } else { 0 }
    };
    if x!= 1 {
        std::process::exit(3);
    }
    if unsafe { G }!= 1 {
        std::process::exit(4);
    }

    std::process::exit(0);
}