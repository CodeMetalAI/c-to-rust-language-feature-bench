fn set_g_return(gval: i32, ret: i32) -> i32 {
    unsafe { g = gval; }
    ret
}

static mut g: i32;

fn main() {
    let mut y = 0;

    unsafe { g = 0; }
    if !set_g_return(1, 0) || (unsafe { g == 1 }) {
        std::process::exit(1);
    }

    unsafe { g = 0; }
    if set_g_return(0, 1) || (y += 1) {
        std::process::exit(2);
    }
    if y != 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}