fn set_g_return(gval: i32, ret: i32) -> i32 {
    let mut g = gval;
    ret
}

fn main() {
    let mut g = 0;
    let mut x;

    g = 0;
    x = if set_g_return(1, 1) != 0 {
        if g == 1 { 1 } else { 0 }
    } else {
        g = 2;
        0
    };
    if x != 1 {
        std::process::exit(1);
    }
    if g != 1 {
        std::process::exit(2);
    }

    g = 0;
    x = if set_g_return(1, 0) != 0 {
        g = 2;
        0
    } else {
        if g == 1 { 1 } else { 0 }
    };
    if x != 1 {
        std::process::exit(3);
    }
    if g != 1 {
        std::process::exit(4);
    }
}