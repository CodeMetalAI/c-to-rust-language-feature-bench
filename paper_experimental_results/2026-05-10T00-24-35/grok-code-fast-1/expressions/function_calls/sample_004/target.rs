use std::cell::RefCell;

thread_local! {
    static SEEN_F1: RefCell<bool> = RefCell::new(false);
    static SEEN_F2: RefCell<bool> = RefCell::new(false);
    static SEEN_F3: RefCell<bool> = RefCell::new(false);
    static SEEN_F4: RefCell<bool> = RefCell::new(false);
    static LOGV: RefCell<Vec<i32>> = RefCell::new(Vec::with_capacity(5));
    static LOGN: RefCell<usize> = RefCell::new(0);
}

fn log_call(id: i32) {
    let mut logn = LOGN.borrow_mut();
    if *logn < 5 {
        let mut logv = LOGV.borrow_mut();
        logv.push(id);
        *logn += 1;
    }
}

fn f1() -> i32 {
    *SEEN_F1.borrow_mut() = true;
    log_call(1);
    0
}

fn f2() -> i32 {
    *SEEN_F2.borrow_mut() = true;
    log_call(2);
    20
}

fn f3() -> i32 {
    *SEEN_F3.borrow_mut() = true;
    log_call(3);
    30
}

fn f4() -> i32 {
    *SEEN_F4.borrow_mut() = true;
    log_call(4);
    40
}

fn target(_a: i32, _b: i32) -> i32 {
    log_call(9);
    if !*SEEN_F1.borrow() || !*SEEN_F2.borrow() || !*SEEN_F3.borrow() || !*SEEN_F4.borrow() {
        100
    } else {
        0
    }
}

fn main() {
    type Fn2 = fn(i32, i32) -> i32;
    let pf: [Fn2; 1] = [target];
    let index = f1() as usize;
    let rc = pf[index](f2(), f3() + f4());
    if rc != 0 {
        std::process::exit(rc);
    }
    let logn = *LOGN.borrow();
    if logn != 5 {
        std::process::exit(1);
    }
    let logv = LOGV.borrow();
    if logv[4] != 9 {
        std::process::exit(2);
    }
    let mut counts: [i32; 10] = [0; 10];
    for i in 0..4 {
        counts[logv[i] as usize] += 1;
    }
    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        std::process::exit(3);
    }
    std::process::exit(0);
}