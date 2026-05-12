static mut G: i32 = 0;

fn set_g_return(gval: i32, ret: i32) -> i32 {
    unsafe {
        G = gval;
    }
    ret
}

fn main() -> i32 {
    let mut x;

    unsafe {
        G = 0;
    }
    x = if set_g_return(1, 1) != 0 { 
        (unsafe { G } == 1) as i32 
    } else { 
        set_g_return(2, 1);
        0 
    };
    if x != 1 {
        return 1;
    }
    if unsafe { G } != 1 {
        return 2;
    }

    unsafe {
        G = 0;
    }
    x = if set_g_return(1, 0) != 0 { 
        set_g_return(2, 1);
        0 
    } else { 
        (unsafe { G } == 1) as i32 
    };
    if x != 1 {
        return 3;
    }
    if unsafe { G } != 1 {
        return 4;
    }

    0
}