use std::cell::{Cell, RefCell};
use std::process::exit;

thread_local! {
    static SEEN_F1: Cell<bool> = Cell::new(false);
    static SEEN_F2: Cell<bool> = Cell::new(false);
    static SEEN_F3: Cell<bool> = Cell::new(false);
    static SEEN_F4: Cell<bool> = Cell::new(false);
    static LOGV: RefCell<Vec<i32>> = RefCell::new(Vec::new());
}

fn log_call(id: i32) {
    LOGV.with(|v| {
        let mut v = v.borrow_mut();
        if v.len() < 5 {
            v.push(id);
        }
    });
}

fn f1() -> i32 {
    SEEN_F1.with(|s| s.set(true));
    log_call(1);
    0
}
fn f2() -> i32 {
    SEEN_F2.with(|s| s.set(true));
    log_call(2);
    20
}
fn f3() -> i32 {
    SEEN_F3.with(|s| s.set(true));
    log_call(3);
    30
}
fn f4() -> i32 {
    SEEN_F4.with(|s| s.set(true));
    log_call(4);
    40
}

type Fn2 = fn(i32, i32) -> i32;

fn target(_a: i32, _b: i32) -> i32 {
    log_call(9);

    let seen = SEEN_F1.with(|s1| s1.get())
        && SEEN_F2.with(|s2| s2.get())
        && SEEN_F3.with(|s3| s3.get())
        && SEEN_F4.with(|s4| s4.get());

    if !seen {
        100
    } else {
        0
    }
}

fn main() {
    let pf: [Fn2; 1] = [target];

    let rc = (pf[f1() as usize])(f2(), f3() + f4());
    if rc != 0 {
        exit(rc);
    }

    let logn = LOGV.with(|v| v.borrow().len());
    if logn != 5 {
        exit(1);
    }
    let last = LOGV.with(|v| v.borrow()[4]);
    if last != 9 {
        exit(2);
    }

    let mut counts = [0i32; 10];
    LOGV.with(|v| {
        let v = v.borrow();
        for i in 0..4 {
            counts[v[i] as usize] += 1;
        }
    });

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        exit(3);
    }

    exit(0);
}