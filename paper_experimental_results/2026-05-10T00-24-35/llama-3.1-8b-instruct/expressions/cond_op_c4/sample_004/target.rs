fn main() {
    let mut g = 0;

    let x = if set_g_return(1, true) {
        g == 1
    } else {
        set_g_return(2, true);
        false
    };
    if x!= true {
        std::process::exit(1);
    }
    if g!= 1 {
        std::process::exit(2);
    }

    g = 0;
    let x = if set_g_return(1, false) {
        set_g_return(2, true);
        false
    } else {
        g == 1
    };
    if x!= true {
        std::process::exit(3);
    }
    if g!= 1 {
        std::process::exit(4);
    }

    std::process::exit(0);
}

fn set_g_return(gval: i32, ret: bool) -> bool {
    g = gval;
    ret
}