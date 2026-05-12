fn set_g_return(gval: i32, ret: i32) -> i32 {
    static mut G: i32 = 0;
    unsafe {
        G = gval;
    }
    ret
}

fn main() {
    let mut y = 0;

    unsafe {
        G = 0;
    }
    if unsafe { set_g_return(0, 0) } & (y += 1)!= 0 {
        std::process::exit(1);
    }
    if y!= 0 {
        std::process::exit(2);
    }

    unsafe {
        G = 0;
    }
    if!(unsafe { set_g_return(1, 1) } & (unsafe { G } == 1))!= 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}