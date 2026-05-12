static mut G: i32 = 0;

fn set_g_return(gval: i32, ret: i32) -> i32 {
    G = gval;
    ret
}

fn main() {
    let mut x;

    G = 0;
    x = if set_g_return(1, 1) != 0 { (G == 1) } else { (set_g_return(2, 1), 0).1 };
    if x != 1 {
        std::process::exit(1);
    }
    if G != 1 {
        std::process::exit(2);
    }

    G = 0;
    x = if set_g_return(1, 0) != 0 { (set_g_return(2, 1), 0).1 } else { (G == 1) };
    if x != 1 {
        std::process::exit(3);
    }
    if G != 1 {
        std::process::exit(4);
    }

    std::process::exit(0);
}