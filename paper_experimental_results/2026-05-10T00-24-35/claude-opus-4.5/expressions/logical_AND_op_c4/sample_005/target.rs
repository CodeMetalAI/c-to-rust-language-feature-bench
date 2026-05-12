fn main() {
    let mut g: i32;
    let mut y: i32 = 0;

    let mut set_g_return = |gval: i32, ret: i32| -> i32 {
        g = gval;
        ret
    };

    g = 0;
    if set_g_return(0, 0) != 0 && { y += 1; y != 0 } {
        std::process::exit(1);
    }
    if y != 0 {
        std::process::exit(2);
    }

    g = 0;
    let first_result = {
        g = 1;
        1
    };
    if !(first_result != 0 && (g == 1)) {
        std::process::exit(3);
    }

    std::process::exit(0);
}