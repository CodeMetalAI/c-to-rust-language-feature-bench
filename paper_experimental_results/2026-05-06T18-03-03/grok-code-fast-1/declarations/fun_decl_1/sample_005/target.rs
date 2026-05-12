use std::cell::{Ref, RefCell};
use std::process;

static GSTORE: RefCell<[i32; 4]> = RefCell::new([0; 4]);

fn f() -> i32 {
    GSTORE.borrow_mut()[0] = 111;
    GSTORE.borrow()[0] + 1
}

fn fip() -> Ref<'static, i32> {
    GSTORE.borrow_mut()[1] = 222;
    GSTORE.borrow().index(1)
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

struct FuncPtr {
    f: fn() -> i32,
    id: i32,
}

const ALT0: FuncPtr = FuncPtr { f: alt0, id: 0 };
const ALT1: FuncPtr = FuncPtr { f: alt1, id: 1 };

fn main() {
    let r_f = f();
    if r_f != 112 {
        process::exit(1);
    }

    let v_fip = *fip();
    if v_fip != 222 {
        process::exit(2);
    }

    let mut pfi = ALT0;
    if choose(GSTORE.borrow()[0]) == 0 {
        pfi = ALT1;
    }

    {
        let r_pfi = (pfi.f)();
        let r_use = use_call_through(pfi.f);

        if r_pfi != r_use {
            process::exit(3);
        }

        if pfi.id == 0 {
            if r_pfi != 332 {
                process::exit(4);
            }
        } else {
            if r_pfi != 446 {
                process::exit(5);
            }
        }
    }

    {
        let q = (pfi.f)();
        if q == 0 {
            process::exit(6);
        }
    }

    process::exit(0);
}