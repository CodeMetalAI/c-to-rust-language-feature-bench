use std::sync::Mutex;

static G: Mutex<i32> = Mutex::new(0);

fn set_g_return(gval: i32, ret: i32) -> i32 {
    *G.lock().unwrap() = gval;
    ret
}

fn main() {
    *G.lock().unwrap() = 0;
    let x = if set_g_return(1, 1) != 0 {
        (*G.lock().unwrap() == 1) as i32
    } else {
        set_g_return(2, 1);
        0
    };
    if x != 1 {
        std::process::exit(1);
    }
    if *G.lock().unwrap() != 1 {
        std::process::exit(2);
    }

    *G.lock().unwrap() = 0;
    let x = if set_g_return(1, 0) != 0 {
        set_g_return(2, 1);
        0
    } else {
        (*G.lock().unwrap() == 1) as i32
    };
    if x != 1 {
        std::process::exit(3);
    }
    if *G.lock().unwrap() != 1 {
        std::process::exit(4);
    }
}