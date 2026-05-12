use std::cell::{Cell, RefCell};

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
    SEEN_F1.with(|c| c.set(true));
    log_call(1);
    0
}

fn f2() -> i32 {
    SEEN_F2.with(|c| c.set(true));
    log_call(2);
    20
}

fn f3() -> i32 {
    SEEN_F3.with(|c| c.set(true));
    log_call(3);
    30
}

fn f4() -> i32 {
    SEEN_F4.with(|c| c.set(true));
    log_call(4);
    40
}

fn target(_a: i32, _b: i32) -> i32 {
    log_call(9);

    let all_seen = SEEN_F1.with(|c| c.get())
        && SEEN_F2.with(|c| c.get())
        && SEEN_F3.with(|c| c.get())
        && SEEN_F4.with(|c| c.get());

    if !all_seen {
        100
    } else {
        0
    }
}

fn main() {
    let pf: [fn(i32, i32) -> i32; 1] = [target];

    let rc = (pf[f1() as usize])(f2(), f3() + f4());
    if rc != 0 {
        std::process::exit(rc);
    }

    let logn = LOGN.with(|n| n.get());
    if logn != 5 {
        std::process::exit(1);
    }

    let last = LOGV.with(|v| v.borrow()[4]);
    if last != 9 {
        std::process::exit(2);
    }

    let mut counts = [0i32; 10];
    for i in 0..4 {
        let id = LOGV.with(|v| v.borrow()[i]) as usize;
        counts[id] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        std::process::exit(3);
    }

    std::process::exit(0);
}