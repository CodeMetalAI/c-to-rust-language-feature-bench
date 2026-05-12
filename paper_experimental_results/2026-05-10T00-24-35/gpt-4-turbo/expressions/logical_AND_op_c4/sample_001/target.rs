fn set_g_return(gval: i32, ret: i32) -> i32 {
    static mut G: i32 = 0;
    unsafe {
        G = gval;
    }
    ret
}

fn main() {
    let mut y = 0;

    if set_g_return(0, 0) != 0 && {
        y += 1;
        y
    } != 0
    {
        std::process::exit(1);
    }
    if y != 0 {
        std::process::exit(2);
    }

    if !(set_g_return(1, 1) != 0 && {
        static mut G: i32 = 0;
        unsafe { G == 1 }
    }) {
        std::process::exit(3);
    }
}