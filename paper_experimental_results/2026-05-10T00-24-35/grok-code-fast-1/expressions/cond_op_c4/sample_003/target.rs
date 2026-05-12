fn set_g_return(g: &mut i32, gval: i32, ret: i32) -> i32 {
    *g = gval;
    ret
}

fn main() {
    let mut g: i32 = 0;
    g = 0;
    let x: i32 = if set_g_return(&mut g, 1, 1) != 0 {
        if *g == 1 { 1 } else { 0 }
    } else {
        set_g_return(&mut g, 2, 1);
        0
    };
    if x != 1 {
        std::process::exit(1);
    }
    if *g != 1 {
        std::process::exit(2);
    }

    g = 0;
    let x: i32 = if set_g_return(&mut g, 1, 0) != 0 {
        set_g_return(&mut g, 2, 1);
        0
    } else {
        if *g == 1 { 1 } else { 0 }
    };
    if x != 1 {
        std::process::exit(3);
    }
    if *g != 1 {
        std::process::exit(4);
    }

    std::process::exit(0);
}