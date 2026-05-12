fn set_g_return(gval: i32, ret: i32) -> i32 {
    unsafe {
        g = gval;
    }
    ret
}

static mut g: i32;

fn main() {
    let mut y = 0;

    unsafe {
        g = 0;
    }
    if set_g_return(0, 0) && (y += 1) {
        return 1;
    }
    if y != 0 {
        return 2;
    }

    unsafe {
        g = 0;
    }
    if !set_g_return(1, 1) && (g == 1) {
        return 3;
    }

    0
}