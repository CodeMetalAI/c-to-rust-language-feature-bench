use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::new());
}

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

fn log_call(id: i32) {
    let mut state = STATE.lock().unwrap();
    if state.logn < 5 {
        state.logv[state.logn] = id;
        state.logn += 1;
    }
}

fn f1() -> i32 {
    let mut state = STATE.lock().unwrap();
    state.seen[0] = true;
    drop(state);
    log_call(1);
    0
}

fn f2() -> i32 {
    let mut state = STATE.lock().unwrap();
    state.seen[1] = true;
    drop(state);
    log_call(2);
    20
}

fn f3() -> i32 {
    let mut state = STATE.lock().unwrap();
    state.seen[2] = true;
    drop(state);
    log_call(3);
    30
}

fn f4() -> i32 {
    let mut state = STATE.lock().unwrap();
    state.seen[3] = true;
    drop(state);
    log_call(4);
    40
}

fn target(_a: i32, _b: i32) -> i32 {
    log_call(9);
    let state = STATE.lock().unwrap();
    for &s in &state.seen {
        if !s {
            return 100;
        }
    }
    0
}

fn main() {
    let pf: [fn(i32, i32) -> i32; 1] = [target];

    let rc = pf[f1() as usize](f2(), f3() + f4());
    if rc != 0 {
        std::process::exit(rc);
    }

    let state = STATE.lock().unwrap();
    if state.logn != 5 {
        std::process::exit(1);
    }
    if state.logv[4] != 9 {
        std::process::exit(2);
    }

    let mut counts = [0; 10];
    for i in 0..4 {
        counts[state.logv[i as usize] as usize] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        std::process::exit(3);
    }
}