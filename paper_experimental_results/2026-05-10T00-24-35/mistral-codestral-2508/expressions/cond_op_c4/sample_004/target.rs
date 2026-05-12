fn main() {
    let mut g = 0;

    fn set_g_return(gval: i32, ret: i32, g: &mut i32) -> i32 {
        *g = gval;
        ret
    }

    g = 0;
    let x = if set_g_return(1, 1, &mut g) != 0 {
        g == 1
    } else {
        set_g_return(2, 1, &mut g);
        false
    };
    if !x {
        std::process::exit(1);
    }
    if g != 1 {
        std::process::exit(2);
    }

    g = 0;
    let x = if set_g_return(1, 0, &mut g) != 0 {
        set_g_return(2, 1, &mut g);
        false
    } else {
        g == 1
    };
    if !x {
        std::process::exit(3);
    }
    if g != 1 {
        std::process::exit(4);
    }
}