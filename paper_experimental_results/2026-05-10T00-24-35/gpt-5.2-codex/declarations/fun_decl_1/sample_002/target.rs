use std::cell::RefCell;

thread_local! {
    static GSTORE: RefCell<[i32; 4]> = RefCell::new([0; 4]);
}

fn f() -> i32 {
    GSTORE.with(|g| {
        let mut gs = g.borrow_mut();
        gs[0] = 111;
        gs[0] + 1
    })
}

fn fip() -> i32 {
    GSTORE.with(|g| {
        let mut gs = g.borrow_mut();
        gs[1] = 222;
        gs[1]
    })
}

fn alt0() -> i32 {
    GSTORE.with(|g| {
        let mut gs = g.borrow_mut();
        gs[2] = 333;
        gs[2] - 1
    })
}

fn alt1() -> i32 {
    GSTORE.with(|g| {
        let mut gs = g.borrow_mut();
        gs[3] = 444;
        gs[3] + 2
    })
}

fn use_call_through(pf: fn() -> i32) -> i32 {
    pf()
}

fn choose(x: i32) -> i32 {
    if x & 1 != 0 {
        1
    } else {
        0
    }
}

fn main() {
    let r_f = f();
    if r_f != 112 {
        std::process::exit(1);
    }

    let v_fip = fip();
    if v_fip != 222 {
        std::process::exit(2);
    }

    let mut pfi: fn() -> i32 = alt0;
    let g0 = GSTORE.with(|g| g.borrow()[0]);
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
        } else if r_pfi != 446 {
            std::process::exit(5);
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