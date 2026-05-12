fn main() {
    let mut g: i32 = 0;
    let set_g_return = |gval: i32, ret: i32| -> i32 {
        g = gval;
        ret
    };

    g = 0;
    let x: i32 = if set_g_return(1, 1) != 0 {
        (g == 1) as i32
    } else {
        set_g_return(2, 1);
        0
    };
    if x != 1 {
        std::process::exit(1);
    }
    if g != 1 {
        std::process::exit(2);
    }

    g = 0;
    let x: i32 = if set_g_return(1, 0) != 0 {
        set_g_return(2, 1);
        0
    } else {
        (g == 1) as i32
    };
    if x != 1 {
        std::process::exit(3);
    }
    if g != 1 {
        std::process::exit(4);
    }

    std::process::exit(0);
}