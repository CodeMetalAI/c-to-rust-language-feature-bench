static mut G: i32 = 0;

fn set_g_return(gval: i32, ret: i32) -> i32 {
    unsafe {
        G = gval;
    }
    ret
}

fn main() -> i32 {
    let mut y = 0;

    unsafe { G = 0; }
    if !(set_g_return(1, 0) != 0 || unsafe { G == 1 }) {
        return 1;
    }

    unsafe { G = 0; }
    if set_g_return(0, 1) != 0 || {
        y += 1;
        y > 0
    } {
        return 2;
    }
    if y != 0 {
        return 3;
    }

    return 0;
}