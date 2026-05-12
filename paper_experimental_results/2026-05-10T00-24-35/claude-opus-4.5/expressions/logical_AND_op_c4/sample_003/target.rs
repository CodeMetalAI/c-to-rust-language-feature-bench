fn main() {
    let mut g: i32;
    let mut y: i32 = 0;

    let mut set_g_return = |gval: i32, ret: i32, g_ref: &mut i32| -> i32 {
        *g_ref = gval;
        ret
    };

    g = 0;
    let first_cond = set_g_return(0, 0, &mut g);
    if first_cond != 0 {
        y += 1;
        if y != 0 {
            std::process::exit(1);
        }
    }
    if y != 0 {
        std::process::exit(2);
    }

    g = 0;
    let second_cond = set_g_return(1, 1, &mut g);
    if !(second_cond != 0 && g == 1) {
        std::process::exit(3);
    }

    std::process::exit(0);
}