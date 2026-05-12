fn set_g_return(gval: i32, ret: i32) -> i32 {
    static mut G: i32 = 0;
    unsafe {
        G = gval;
        ret
    }
}

fn main() {
    let mut y = 0;

    unsafe {
        set_g_return(0, 0);
    }
    if!(unsafe { set_g_return(1, 0) }!= 0 || (unsafe { set_g_return(0, 0) } == 1)) {
        return;
    }

    unsafe {
        set_g_return(0, 0);
    }
    if (unsafe { set_g_return(0, 1) }!= 0 || (y += 1)!= 0) {
        return;
    }
    if y!= 0 {
        return;
    }

    std::process::exit(0);
}