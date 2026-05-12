use std::cell::RefCell;

thread_local! {
    static SEEN_F1: RefCell<bool> = RefCell::new(false);
    static SEEN_F2: RefCell<bool> = RefCell::new(false);
    static SEEN_F3: RefCell<bool> = RefCell::new(false);
    static SEEN_F4: RefCell<bool> = RefCell::new(false);
    static LOGV: RefCell<[i32; 5]> = RefCell::new([0; 5]);
    static LOGN: RefCell<usize> = RefCell::new(0);
}

fn log_call(id: i32) {
    LOGN.with(|n| {
        let mut n = n.borrow_mut();
        if *n < 5 {
            LOGV.with(|v| {
                let mut v = v.borrow_mut();
                v[*n] = id;
            });
            *n += 1;
        }
    });
}

fn f1() -> i32 {
    SEEN_F1.with(|s| *s.borrow_mut() = true);
    log_call(1);
    0
}

fn f2() -> i32 {
    SEEN_F2.with(|s| *s.borrow_mut() = true);
    log_call(2);
    20
}

fn f3() -> i32 {
    SEEN_F3.with(|s| *s.borrow_mut() = true);
    log_call(3);
    30
}

fn f4() -> i32 {
    SEEN_F4.with(|s| *s.borrow_mut() = true);
    log_call(4);
    40
}

fn target(_a: i32, _b: i32) -> i32 {
    log_call(9);
    let seen = SEEN_F1.with(|s| *s.borrow())
        && SEEN_F2.with(|s| *s.borrow())
        && SEEN_F3.with(|s| *s.borrow())
        && SEEN_F4.with(|s| *s.borrow());
    if !seen { 100 } else { 0 }
}

fn main() {
    type Fn2T = fn(i32, i32) -> i32;
    let pf: [Fn2T; 1] = [target];
    let rc = pf[f1() as usize](f2(), f3() + f4());
    if rc != 0 {
        std::process::exit(rc);
    }
    let logn = LOGN.with(|n| *n.borrow());
    if logn != 5 {
        std::process::exit(1);
    }
    let logv = LOGV.with(|v| *v.borrow());
    if logv[4] != 9 {
        std::process::exit(2);
    }
    let mut counts = [0i32; 10];
    for &id in &logv[0..4] {
        counts[id as usize] += 1;
    }
    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        std::process::exit(3);
    }
    std::process::exit(0);
}