struct Global {
    g: i32,
}

impl Global {
    fn set_g_return(&mut self, gval: i32, ret: i32) -> i32 {
        self.g = gval;
        ret
    }
}

fn main() {
    let mut global = Global { g: 0 };
    let mut y = 0;

    global.g = 0;
    if global.set_g_return(0, 0) != 0 && { y += 1; true } {
        std::process::exit(1);
    }
    if y != 0 {
        std::process::exit(2);
    }

    global.g = 0;
    if !(global.set_g_return(1, 1) != 0 && global.g == 1) {
        std::process::exit(3);
    }

    std::process::exit(0);
}