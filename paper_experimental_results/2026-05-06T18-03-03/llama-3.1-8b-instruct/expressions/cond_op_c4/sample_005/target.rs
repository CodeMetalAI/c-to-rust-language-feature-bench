fn main() {
    let mut g = 0;

    let x = if set_g_return(1, 1) { g == 1 } else { set_g_return(2, 1); 0 };
    if x!= 1 {
        return 1;
    }
    if g!= 1 {
        return 2;
    }

    g = 0;
    let x = if set_g_return(1, 0) { set_g_return(2, 1); 0 } else { g == 1 };
    if x!= 1 {
        return 3;
    }
    if g!= 1 {
        return 4;
    }

    0
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    g = gval;
    ret
}