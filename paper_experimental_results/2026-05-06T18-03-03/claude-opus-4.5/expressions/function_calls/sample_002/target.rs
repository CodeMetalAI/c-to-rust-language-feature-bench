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
    LOGN.with(|logn| {
        LOGV.with(|logv| {
            let mut n = logn.borrow_mut();
            if *n < 5 {
                logv.borrow_mut()[*n] = id;
                *n += 1;
            }
        });
    });
}

fn f1() -> usize {
    SEEN_F1.with(|seen| *seen.borrow_mut() = true);
    log_call(1);
    0
}

fn f2() -> i32 {
    SEEN_F2.with(|seen| *seen.borrow_mut() = true);
    log_call(2);
    20
}

fn f3() -> i32 {
    SEEN_F3.with(|seen| *seen.borrow_mut() = true);
    log_call(3);
    30
}

fn f4() -> i32 {
    SEEN_F4.with(|seen| *seen.borrow_mut() = true);
    log_call(4);
    40
}

fn target(_a: i32, _b: i32) -> i32 {
    log_call(9);
    
    let all_seen = SEEN_F1.with(|s| *s.borrow())
        && SEEN_F2.with(|s| *s.borrow())
        && SEEN_F3.with(|s| *s.borrow())
        && SEEN_F4.with(|s| *s.borrow());
    
    if !all_seen {
        return 100;
    }
    
    0
}

fn main() {
    let pf: [fn(i32, i32) -> i32; 1] = [target];
    
    let idx = f1();
    let arg1 = f2();
    let arg2 = f3() + f4();
    let rc = pf[idx](arg1, arg2);
    
    if rc != 0 {
        std::process::exit(rc);
    }
    
    let logn = LOGN.with(|n| *n.borrow());
    if logn != 5 {
        std::process::exit(1);
    }
    
    let logv4 = LOGV.with(|v| v.borrow()[4]);
    if logv4 != 9 {
        std::process::exit(2);
    }
    
    let mut counts = [0i32; 10];
    LOGV.with(|logv| {
        let v = logv.borrow();
        for i in 0..4 {
            counts[v[i] as usize] += 1;
        }
    });
    
    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        std::process::exit(3);
    }
}