use std::process::exit;
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
    pf()
}

fn choose(x: i32) -> i32 {
    if x & 1 != 0 { 1 } else { 0 }
}

fn get_gstore0() -> i32 {
    let g = GSTORE.lock().unwrap();
    g[0]
}

fn main() {
    let r_f = f();
    if r_f != 112 {
        exit(1);
    }

    let v_fip = fip();
    if v_fip != 222 {
        exit(2);
    }

    let mut pfi: fn() -> i32 = alt0;
    if choose(get_gstore0()) == 0 {
        pfi = alt1;
    }

    {
        let r_pfi = pfi();
        let r_use = use_call_through(pfi);

        if r_pfi != r_use {
            exit(3);
        }

        if pfi as usize == alt0 as usize {
            if r_pfi != 332 {
                exit(4);
            }
        } else {
            if r_pfi != 446 {
                exit(5);
            }
        }
    }

    {
        let q = pfi();
        if q == 0 {
            exit(6);
        }
    }

    exit(0);
}