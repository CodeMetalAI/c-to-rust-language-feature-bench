extern crate lazy_static;

use lazy_static::lazy_static;
use std::cell::RefCell;

lazy_static! {
    static ref GSTORE: RefCell<[i32; 4]> = RefCell::new([0; 4]);
}

fn f() -> i32 {
    GSTORE.borrow_mut()[0] = 111;
    GSTORE.borrow()[0] + 1
}

fn fip() -> i32 {
    GSTORE.borrow_mut()[1] = 222;
    GSTORE.borrow()[1]
}

fn alt0() -> i32 {
    GSTORE.borrow_mut()[2] = 333;
    GSTORE.borrow()[2] - 1
}

fn alt1() -> i32 {
    GSTORE.borrow_mut()[3] = 444;
    GSTORE.borrow()[3] + 2
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
    if choose(GSTORE.borrow()[0]) == 0 {
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