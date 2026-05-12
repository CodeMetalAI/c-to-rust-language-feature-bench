use std::cell::{Cell, RefCell};
use std::process::exit;

thread_local! {
    static SEEN_F1: Cell<bool> = Cell::new(false);
    static SEEN_F2: Cell<bool> = Cell::new(false);
    static SEEN_F3: Cell<bool> = Cell::new(false);
    static SEEN_F4: Cell<bool> = Cell::new(false);
    static LOGV: RefCell<[i32; 5]> = RefCell::new([0; 5]);
    static LOGN: Cell<usize> = Cell::new(0);
}

fn log_call(id: i32) {
    LOGN.with(|n| {
        let idx = n.get();
        if idx < 5 {
            LOGV.with(|v| {
                v.borrow_mut()[idx] = id;
            });
            n.set(idx + 1);
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

    let all_seen = SEEN_F1.with(|s| s.get())
        && SEEN_F2.with(|s| s.get())
        && SEEN_F3.with(|s| s.get())
        && SEEN_F4.with(|s| s.get());

    if !all_seen {
        return 100;
    }
    0
}

fn main() {
    let pf: [Fn2; 1] = [target];

    let rc = (pf[f1() as usize])(f2(), f3() + f4());
    if rc != 0 {
        exit(rc);
    }

    let logn = LOGN.with(|n| n.get());
    if logn != 5 {
        exit(1);
    }

    let logv = LOGV.with(|v| *v.borrow());
    if logv[4] != 9 {
        exit(2);
    }

    let mut counts = [0i32; 10];
    for i in 0..4 {
        let idx = logv[i] as usize;
        counts[idx] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        exit(3);
    }

    exit(0);
}