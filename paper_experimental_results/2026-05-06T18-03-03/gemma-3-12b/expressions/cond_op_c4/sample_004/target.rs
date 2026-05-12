fn set_g_return(gval: i32, ret: i32) -> i32 {
    static mut g: i32 = 0;
    unsafe {
        g = gval;
        ret
    }
}

fn main() {
    static mut g: i32 = 0;

    g = 0;
    let x = if set_g_return(1, 1) != 0 {
        g == 1
    } else {
        set_g_return(2, 1);
        0
    };

    if x != 1 {
        std::process::exit(1);
    }
    unsafe {
        if g != 1 {
            std::process::exit(2);
        }
    }

    g = 0;
    let x = if set_g_return(1, 0) != 0 {
        set_g_return(2, 1);
        0
    } else {
        g == 1
    };

    if x != 1 {
        std::process::exit(3);
    }
    unsafe {
        if g != 1 {
            std::process::exit(4);
        }
    }

    0
}