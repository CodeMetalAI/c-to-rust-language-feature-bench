fn main() {
    static mut G: isize = 0;

    fn set_g_return(gval: isize, ret: isize) -> isize {
        unsafe {
            G = gval;
        }
        ret
    }

    let mut y = 0;

    unsafe {
        G = 0;
    }
    if set_g_return(0, 0) != 0 && { y += 1; y != 0 } {
        std::process::exit(1);
    }
    if y != 0 {
        std::process::exit(2);
    }

    unsafe {
        G = 0;
    }
    if !(set_g_return(1, 1) != 0 && unsafe { G == 1 }) {
        std::process::exit(3);
    }
}