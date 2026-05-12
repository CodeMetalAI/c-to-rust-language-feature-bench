use std::process::exit;
use std::sync::{Mutex, OnceLock};

static G: OnceLock<Mutex<i32>> = OnceLock::new();

fn set_g(val: i32) {
    let m = G.get_or_init(|| Mutex::new(0));
    let mut guard = m.lock().unwrap();
    *guard = val;
}

fn get_g() -> i32 {
    let m = G.get_or_init(|| Mutex::new(0));
    let guard = m.lock().unwrap();
    *guard
}

fn set_g_return(gval: i32, ret: i32) -> i32 {
    set_g(gval);
    ret
}

fn main() {
    let mut y = 0;

    set_g(0);
    if !(set_g_return(1, 0) != 0 || get_g() == 1) {
        exit(1);
    }

    set_g(0);
    if set_g_return(0, 1) != 0 || {
        y += 1;
        y != 0
    } {
        exit(2);
    }
    if y != 0 {
        exit(3);
    }

    exit(0);
}