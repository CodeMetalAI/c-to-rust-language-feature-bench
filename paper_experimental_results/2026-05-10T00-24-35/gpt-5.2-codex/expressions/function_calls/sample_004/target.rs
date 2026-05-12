use std::cell::RefCell;
use std::process::exit;

struct State {
    seen_f1: bool,
    seen_f2: bool,
    seen_f3: bool,
    seen_f4: bool,
    logv: [i32; 5],
    logn: usize,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        seen_f1: false,
        seen_f2: false,
        seen_f3: false,
        seen_f4: false,
        logv: [0; 5],
        logn: 0,
    });
}

fn log_call(id: i32) {
    STATE.with(|s| {
        let mut st = s.borrow_mut();
        if st.logn < 5 {
            st.logv[st.logn] = id;
            st.logn += 1;
        }
    });
}

fn f1() -> i32 {
    STATE.with(|s| s.borrow_mut().seen_f1 = true);
    log_call(1);
    0
}

fn f2() -> i32 {
    STATE.with(|s| s.borrow_mut().seen_f2 = true);
    log_call(2);
    20
}

fn f3() -> i32 {
    STATE.with(|s| s.borrow_mut().seen_f3 = true);
    log_call(3);
    30
}

fn f4() -> i32 {
    STATE.with(|s| s.borrow_mut().seen_f4 = true);
    log_call(4);
    40
}

type Fn2 = fn(i32, i32) -> i32;

fn target(_a: i32, _b: i32) -> i32 {
    log_call(9);
    let all_seen = STATE.with(|s| {
        let st = s.borrow();
        st.seen_f1 && st.seen_f2 && st.seen_f3 && st.seen_f4
    });
    if !all_seen {
        100
    } else {
        0
    }
}

fn get_logn() -> usize {
    STATE.with(|s| s.borrow().logn)
}

fn get_logv() -> [i32; 5] {
    STATE.with(|s| s.borrow().logv)
}

fn main() {
    let pf: [Fn2; 1] = [target];

    let rc = (pf[f1() as usize])(f2(), f3() + f4());
    if rc != 0 {
        exit(rc);
    }

    let logn = get_logn();
    if logn != 5 {
        exit(1);
    }
    let logv = get_logv();
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