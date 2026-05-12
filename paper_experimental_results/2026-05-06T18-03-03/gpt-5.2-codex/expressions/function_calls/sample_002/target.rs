use std::cell::RefCell;

#[derive(Clone, Copy)]
struct State {
    seen: [bool; 4],
    logv: [i32; 5],
    logn: usize,
}

impl State {
    fn new() -> Self {
        State {
            seen: [false; 4],
            logv: [0; 5],
            logn: 0,
        }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
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
    STATE.with(|s| {
        s.borrow_mut().seen[0] = true;
    });
    log_call(1);
    0
}

fn f2() -> i32 {
    STATE.with(|s| {
        s.borrow_mut().seen[1] = true;
    });
    log_call(2);
    20
}

fn f3() -> i32 {
    STATE.with(|s| {
        s.borrow_mut().seen[2] = true;
    });
    log_call(3);
    30
}

fn f4() -> i32 {
    STATE.with(|s| {
        s.borrow_mut().seen[3] = true;
    });
    log_call(4);
    40
}

fn target(_a: i32, _b: i32) -> i32 {
    log_call(9);

    let all_seen = STATE.with(|s| s.borrow().seen.iter().all(|&x| x));
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

    let (logn, logv) = STATE.with(|s| {
        let st = s.borrow();
        (st.logn, st.logv)
    });

    if logn != 5 {
        std::process::exit(1);
    }
    if logv[4] != 9 {
        std::process::exit(2);
    }

    let mut counts = [0i32; 10];
    for i in 0..4 {
        let id = logv[i] as usize;
        counts[id] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        std::process::exit(3);
    }
}