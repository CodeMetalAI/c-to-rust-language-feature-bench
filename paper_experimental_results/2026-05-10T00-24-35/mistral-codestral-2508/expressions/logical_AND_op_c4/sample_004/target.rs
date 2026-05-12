fn main() {
    let mut g = 0;

    let mut set_g_return = |gval: i32, ret: i32| -> i32 {
        g = gval;
        ret
    };

    let mut y = 0;

    g = 0;
    if set_g_return(0, 0) != 0 && {
        y += 1;
        true
    } {
        std::process::exit(1);
    }
    if y != 0 {
        std::process::exit(2);
    }

    g = 0;
    if !(set_g_return(1, 1) != 0 && g == 1) {
        std::process::exit(3);
    }
}