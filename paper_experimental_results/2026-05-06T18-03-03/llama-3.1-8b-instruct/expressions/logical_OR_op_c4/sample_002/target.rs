fn main() {
    let mut g = 0;
    let mut y = 0;

    if !set_g_return(1, 0) || g != 1 {
        return 1;
    }

    g = 0;
    if set_g_return(0, 1) || y += 1 != 0 {
        return 2;
    }
    if y != 0 {
        return 3;
    }

    0
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    gval
}