use std::cell::RefCell;
use std::process;

#[derive(Clone)]
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

fn target(a: i32, b: i32) -> i32 {
    let _ = a;
    let _ = b;
    log_call(9);
    let ok = STATE.with(|s| {
        let st = s.borrow();
        st.seen_f1 && st.seen_f2 && st.seen_f3 && st.seen_f4
    });
    if !ok {
        100
    } else {
        0
    }
}

fn main() {
    let pf: [fn(i32, i32) -> i32; 1] = [target];

    let rc = pf[f1() as usize](f2(), f3() + f4());
    if rc != 0 {
        process::exit(rc);
    }

    let (logn, logv) = STATE.with(|s| {
        let st = s.borrow();
        (st.logn, st.logv)
    });

    if logn != 5 {
        process::exit(1);
    }
    if logv[4] != 9 {
        process::exit(2);
    }

    let mut counts = [0i32; 10];
    for i in 0..4 {
        let idx = logv[i] as usize;
        counts[idx] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        process::exit(3);
    }

    process::exit(0);
}