use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref G: Mutex<i32> = Mutex::new(0);
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    *G.lock().unwrap() = gval;
    ret
}

fn main() {
    let mut y = 0;

    *G.lock().unwrap() = 0;
    if !(set_g_return(1, 0) != 0 || *G.lock().unwrap() == 1) {
        std::process::exit(1);
    }

    *G.lock().unwrap() = 0;
    if set_g_return(0, 1) != 0 || {
        y += 1;
        y != 0
    } {
        std::process::exit(2);
    }
    if y != 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}