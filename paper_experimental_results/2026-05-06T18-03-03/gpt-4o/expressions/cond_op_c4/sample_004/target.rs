/*
 * cond_op_c4
 */

static mut G: i32 = 0;

fn set_g_return(gval: i32, ret: i32) -> i32 {
    unsafe {
        G = gval;
    }
    ret
}

fn main() {
    let mut x;

    unsafe {
        G = 0;
    }
    x = if set_g_return(1, 1) != 0 {
        unsafe { G == 1 }
    } else {
        set_g_return(2, 1);
        false
    };
    if x != true {
        std::process::exit(1);
    }
    if unsafe { G != 1 } {
        std::process::exit(2);
    }

    unsafe {
        G = 0;
    }
    x = if set_g_return(1, 0) != 0 {
        set_g_return(2, 1);
        false
    } else {
        unsafe { G == 1 }
    };
    if x != true {
        std::process::exit(3);
    }
    if unsafe { G != 1 } {
        std::process::exit(4);
    }

    std::process::exit(0);
}