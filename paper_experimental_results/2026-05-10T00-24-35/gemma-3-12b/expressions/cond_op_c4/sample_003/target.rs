fn set_g_return(gval: i32, ret: i32) -> i32 {
    unsafe { g = gval; }
    ret
}

static mut g: i32;

fn main() -> i32 {
    g = 0;
    let x = if set_g_return(1, 1) != 0 {
        unsafe { g == 1 }
    } else {
        unsafe { set_g_return(2, 1); }
        0
    };

    if x != 1 {
        return 1;
    }

    if unsafe { g != 1 } {
        return 2;
    }

    g = 0;
    let x = if set_g_return(1, 0) != 0 {
        unsafe { set_g_return(2, 1); }
        0
    } else {
        unsafe { g == 1 }
    };

    if x != 1 {
        return 3;
    }

    if unsafe { g != 1 } {
        return 4;
    }

    0
}