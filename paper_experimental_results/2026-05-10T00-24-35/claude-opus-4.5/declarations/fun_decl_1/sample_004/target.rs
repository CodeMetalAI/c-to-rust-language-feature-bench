use std::cell::RefCell;

thread_local! {
    static GSTORE: RefCell<[i32; 4]> = RefCell::new([0; 4]);
}

fn f() -> i32 {
    GSTORE.with(|g| {
        g.borrow_mut()[0] = 111;
        g.borrow()[0] + 1
    })
}

fn fip() -> i32 {
    GSTORE.with(|g| {
        g.borrow_mut()[1] = 222;
        g.borrow()[1]
    })
}

fn alt0() -> i32 {
    GSTORE.with(|g| {
        g.borrow_mut()[2] = 333;
        g.borrow()[2] - 1
    })
}

fn alt1() -> i32 {
    GSTORE.with(|g| {
        g.borrow_mut()[3] = 444;
        g.borrow()[3] + 2
    })
}

fn use_call_through(pf: fn() -> i32) -> i32 {
    let r = pf();
    r
}

fn choose(x: i32) -> i32 {
    if x & 1 != 0 {
        return 1;
    }
    0
}

fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    let pfi: fn() -> i32;

    let r_f = f();
    if r_f != 112 {
        return 1;
    }

    let v_fip = fip();
    if v_fip != 222 {
        return 2;
    }

    let gstore_0 = GSTORE.with(|g| g.borrow()[0]);
    if choose(gstore_0) == 0 {
        pfi = alt1;
    } else {
        pfi = alt0;
    }

    {
        let r_pfi = pfi();
        let r_use = use_call_through(pfi);

        if r_pfi != r_use {
            return 3;
        }

        if pfi == alt0 {
            if r_pfi != 332 {
                return 4;
            }
        } else {
            if r_pfi != 446 {
                return 5;
            }
        }
    }

    {
        let q = pfi();
        if q == 0 {
            return 6;
        }
    }

    0
}