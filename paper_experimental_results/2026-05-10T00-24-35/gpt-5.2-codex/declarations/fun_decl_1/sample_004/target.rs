use std::sync::Mutex;

static GSTORE: Mutex<[i32; 4]> = Mutex::new([0; 4]);

fn f() -> i32 {
    let mut g = GSTORE.lock().unwrap();
    g[0] = 111;
    g[0] + 1
}

fn fip() -> i32 {
    let mut g = GSTORE.lock().unwrap();
    g[1] = 222;
    g[1]
}

fn alt0() -> i32 {
    let mut g = GSTORE.lock().unwrap();
    g[2] = 333;
    g[2] - 1
}

fn alt1() -> i32 {
    let mut g = GSTORE.lock().unwrap();
    g[3] = 444;
    g[3] + 2
}

fn use_call_through(pf: fn() -> i32) -> i32 {
    let r = pf();
    r
}

fn choose(x: i32) -> i32 {
    if x & 1 != 0 {
        1
    } else {
        0
    }
}

fn main() {
    let mut pfi: fn() -> i32;

    let r_f = f();
    if r_f != 112 {
        std::process::exit(1);
    }

    let v_fip = fip();
    if v_fip != 222 {
        std::process::exit(2);
    }

    pfi = alt0;
    let g0 = {
        let g = GSTORE.lock().unwrap();
        g[0]
    };
    if choose(g0) == 0 {
        pfi = alt1;
    }

    {
        let r_pfi = pfi();
        let r_use = use_call_through(pfi);

        if r_pfi != r_use {
            std::process::exit(3);
        }

        if pfi == alt0 {
            if r_pfi != 332 {
                std::process::exit(4);
            }
        } else {
            if r_pfi != 446 {
                std::process::exit(5);
            }
        }
    }

    {
        let q = pfi();
        if q == 0 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}