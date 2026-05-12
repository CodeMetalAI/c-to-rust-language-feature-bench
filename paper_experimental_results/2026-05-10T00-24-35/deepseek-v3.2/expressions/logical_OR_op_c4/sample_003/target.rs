static mut G: i32 =63;

fn set_g_return(gval: i32, ret: i32) -> i32 {
    unsafe {
        G = gval;
    }
    ret
}

fn main() {
    let mut y = 0;

    unsafe {
        G = 0;
    }
    if !(set_g_return(1, 0) != 0 || unsafe { G == 1 }) {
        return;
    }

    unsafe {
        G = 0;
    }
    if set_g_return(0, 1) != 0 || {
        y += 1;
        y != 0
    } {
        return;
    }
    if y != 0 {
        return;
    }
}