fn main() {
    let mut g = 0;
    let mut y = 0;

    g = 0;
    if !(set_g_return(&mut g, 1, 0) || (g == 1)) {
        std::process::exit(1);
    }

    g = 0;
    if set_g_return(&mut g, 0, 1) || {
        y += 1;
        true
    } {
        std::process::exit(2);
    }
    if y != 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}

fn set_g_return(g: &mut i32, gval: i32, ret: i32) -> i32 {
    *g = gval;
    ret
}