use std::sync::Mutex;

static G: Mutex<i32> = Mutex::new(0);

fn set_g_return(gval: i32, ret: i32) -> i32 {
    let mut g = G.lock().unwrap();
    *g = gval;
    ret
}

fn get_g() -> i32 {
    *G.lock().unwrap()
}

fn set_g(gval: i32) {
    let mut g = G.lock().unwrap();
    *g = gval;
}

fn main() {
    let mut x: i32;

    set_g(0);
    x = if set_g_return(1, 1) != 0 {
        if get_g() == 1 { 1 } else { 0 }
    } else {
        set_g_return(2, 1);
        0
    };
    if x != 1 {
        std::process::exit(1);
    }
    if get_g() != 1 {
        std::process::exit(2);
    }

    set_g(0);
    x = if set_g_return(1, 0) != 0 {
        set_g_return(2, 1);
        0
    } else {
        if get_g() == 1 { 1 } else { 0 }
    };
    if x != 1 {
        std::process::exit(3);
    }
    if get_g() != 1 {
        std::process::exit(4);
    }

    std::process::exit(0);
}