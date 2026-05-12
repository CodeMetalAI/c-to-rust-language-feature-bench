fn main() {
    let mut g = 0;
    let mut y = 0;

    g = 0;
    if set_g_return(&mut g, 0, 0) && {
        y += 1;
        true
    } {
        std::process::exit(1);
    }
    if y != 0 {
        std::process::exit(2);
    }

    g = 0;
    if !(set_g_return(&mut g, 1, 1) && g == 1) {
        std::process::exit(3);
    }
}

fn set_g_return(g: &mut i32, gval: i32, ret: i32) -> bool {
    *g = gval;
    ret != 0
}