use std::cell::RefCell;

struct State {
    seen_f1: bool,
    seen_f2: bool,
    seen_f3: bool,
    seen_f4: bool,
    logv: Vec<i32>,
}

static STATE: RefCell<State> = RefCell::new(State {
    seen_f1: false,
    seen_f2: false,
    seen_f3: false,
    seen_f4: false,
    logv: Vec::with_capacity(5),
});

fn log_call(id: i32) {
    let mut state = STATE.borrow_mut();
    if state.logv.len() < 5 {
        state.logv.push(id);
    }
}

fn f1() -> i32 {
    STATE.borrow_mut().seen_f1 = true;
    log_call(1);
    0
}

fn f2() -> i32 {
    STATE.borrow_mut().seen_f2 = true;
    log_call(2);
    20
}

fn f3() -> i32 {
    STATE.borrow_mut().seen_f3 = true;
    log_call(3);
    30
}

fn f4() -> i32 {
    STATE.borrow_mut().seen_f4 = true;
    log_call(4);
    40
}

fn target(_a: i32, _b: i32) -> i32 {
    log_call(9);
    let state = STATE.borrow();
    if !state.seen_f1 || !state.seen_f2 || !state.seen_f3 || !state.seen_f4 {
        100
    } else {
        0
    }
}

fn main() {
    let pf: [fn(i32, i32) -> i32; 1] = [target];
    let rc = pf[f1() as usize](f2(), f3() + f4());
    if rc != 0 {
        std::process::exit(rc);
    }
    let state = STATE.borrow();
    if state.logv.len() != 5 {
        std::process::exit(1);
    }
    if state.logv[4] != 9 {
        std::process::exit(2);
    }
    let mut counts = [0i32; 10];
    for &id in &state.logv[0..4] {
        counts[id as usize] += 1;
    }
    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        std::process::exit(3);
    }
    std::process::exit(0);
}