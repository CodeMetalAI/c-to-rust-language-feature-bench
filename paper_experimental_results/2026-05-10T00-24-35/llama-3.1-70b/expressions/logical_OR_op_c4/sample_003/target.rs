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
        let mut g = 0;
        g = 0;
        if!(set_g_return(1, 0)!= 0 || (g == 1)) {
            return;
        }

        g = 0;
        if set_g_return(0, 1)!= 0 || ({ y += 1; y!= 0 }) {
            std::process::exit(2);
        }
        if y!= 0 {
            std::process::exit(3);
        }
    }

    std::process::exit(0);
}