fn main() {
    let mut g: i32 = 0;
    let y: i32 = 0;

    g = 0;
    if set_g_return(0, 0) && y.saturating_add(1) != 0 {
        return 1;
    }
    if y != 0 {
        return 2;
    }

    g = 0;
    if !(set_g_return(1, 1) && g == 1) {
        return 3;
    }

    return 0;
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    g = gval;
    return ret;
}