static mut G: i32 = 0;

fn set_g_return(gval: i32, ret: i32) -> i32 {
    unsafe {
        G = gval;
    }
    ret
}

fn main() {
    let x: i32;

    unsafe {
        G = 0;
    }
    x = if set_g_return(1, 1) != 0 {
        unsafe { (G == 1) as i32 }
    } else {
        let _ = set_g_return(2, 1);
        0
    };
    if x != 1 {
        std::process::exit(1);
    }
    unsafe {
        if G != 1 {
            std::process::exit(2);
        }
    }

    unsafe {
        G = 0;
    }
    x = if set_g_return(1, 0) != 0 {
        let _ = set_g_return(2, 1);
        0
    } else {
        unsafe { (G == 1) as i32 }
    };
    if x != 1 {
        std::process::exit(3);
    }
    unsafe {
        if G != 1 {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}