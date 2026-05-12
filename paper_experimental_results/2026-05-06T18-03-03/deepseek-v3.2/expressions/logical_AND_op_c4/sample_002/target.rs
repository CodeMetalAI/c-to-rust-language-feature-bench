static mut G: i32 = 0;

fn set_g_return(gval: i32, ret: i32) -> i32 {
    G = gval;
    ret
}

fn main() {
    let mut y = 0;

    G = 0;
    if set_g_return(0, 0) != 0 && {
        y += 1;
        y != 0
    } {
        return;
    }
    if y != 0 {
        return;
    }

    G = 0;
    if !(set_g_return(1, 1) != 0 && G == 1) {
        return;
    }
}