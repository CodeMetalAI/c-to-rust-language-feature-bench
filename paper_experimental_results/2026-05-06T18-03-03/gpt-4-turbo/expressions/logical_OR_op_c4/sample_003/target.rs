fn main() {
    static mut G: i32 = 0;

    fn set_g_return(gval: i32, ret: i32) -> i32 {
        unsafe {
            G = gval;
        }
        ret
    }

    let mut y = 0;

    unsafe {
        G = 0;
    }
    if !(set_g_return(1, 0) != 0 || unsafe { G } == 1) {
        std::process::exit(1);
    }

    unsafe {
        G = 0;
    }
    if set_g_return(0, 1) != 0 || { y += 1; y } > 1 {
        std::process::exit(2);
    }
    if y != 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}