use std::sync::Mutex;

static G: Mutex<i32> = Mutex::new(0);

fn set_g_return(gval: i32, ret: i32) -> i32 {
    let mut g = G.lock().unwrap();
    *g = gval;
    ret
}

fn main() {
    let mut y = 0;

    {
        let mut g = G.lock().unwrap();
        *g = 0;
    }

    if !(set_g_return(1, 0) != 0 || {
        let g = G.lock().unwrap();
        *g == 1
    }) {
        std::process::exit(1);
    }

    {
        let mut g = G.lock().unwrap();
        *g = 0;
    }

    if set_g_return(0, 1) != 0 || {
        y += 1;
        true
    } {
        std::process::exit(2);
    }

    if y != 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}