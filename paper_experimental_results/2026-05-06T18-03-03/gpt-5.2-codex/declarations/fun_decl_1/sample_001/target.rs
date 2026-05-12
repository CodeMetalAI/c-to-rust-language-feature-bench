use std::cell::RefCell;
use std::process;

thread_local! {
    static GSTORE: RefCell<[i32; 4]> = RefCell::new([0; 4]);
}

fn f() -> i32 {
    GSTORE.with(|g| {
        let mut arr = g.borrow_mut();
        arr[0] = 111;
        arr[0] + 1
    })
}

fn fip() -> i32 {
    GSTORE.with(|g| {
        let mut arr = g.borrow_mut();
        arr[1] = 222;
        arr[1]
    })
}

fn alt0() -> i32 {
    GSTORE.with(|g| {
        let mut arr = g.borrow_mut();
        arr[2] = 333;
        arr[2] - 1
    })
}

fn alt1() -> i32 {
    GSTORE.with(|g| {
        let mut arr = g.borrow_mut();
        arr[3] = 444;
        arr[3] + 2
    })
}

fn use_call_through(pf: fn() -> i32) -> i32 {
    pf()
}

fn choose(x: i32) -> i32 {
    if x & 1 != 0 { 1 } else { 0 }
}

fn gstore0() -> i32 {
    GSTORE.with(|g| g.borrow()[0])
}

fn run() -> i32 {
    let r_f = f();
    if r_f != 112 {
        return 1;
    }

    let v_fip = fip();
    if v_fip != 222 {
        return 2;
    }

    let mut pfi: fn() -> i32 = alt0;
    if choose(gstore0()) == 0 {
        pfi = alt1;
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

fn main() {
    let code = run();
    process::exit(code);
}