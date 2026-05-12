fn main() {
    let mut g = 0;

    let set_g_return = |gval, ret| {
        g = gval;
        ret
    };

    let x = set_g_return(1, 1) && g == 1 || set_g_return(2, 1) && 0;
    if x != 1 {
        return 1;
    }
    if g != 1 {
        return 2;
    }

    g = 0;
    let x = set_g_return(1, 0) && set_g_return(2, 1) && 0 || g == 1;
    if x != 1 {
        return 3;
    }
    if g != 1 {
        return 4;
    }

    return 0;
}