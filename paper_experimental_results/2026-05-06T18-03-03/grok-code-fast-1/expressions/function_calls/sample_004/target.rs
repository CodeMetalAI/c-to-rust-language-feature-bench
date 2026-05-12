struct State {
    seen_f1: bool,
    seen_f2: bool,
    seen_f3: bool,
    seen_f4: bool,
    log: Vec<i32>,
}

fn log_call(state: &mut State, id: i32) {
    if state.log.len() < 5 {
        state.log.push(id);
    }
}

fn f1(state: &mut State) -> i32 {
    state.seen_f1 = true;
    log_call(state, 1);
    0
}

fn f2(state: &mut State) -> i32 {
    state.seen_f2 = true;
    log_call(state, 2);
    20
}

fn f3(state: &mut State) -> i32 {
    state.seen_f3 = true;
    log_call(state, 3);
    30
}

fn f4(state: &mut State) -> i32 {
    state.seen_f4 = true;
    log_call(state, 4);
    40
}

fn target(state: &mut State, _a: i32, _b: i32) -> i32 {
    log_call(state, 9);
    if !state.seen_f1 || !state.seen_f2 || !state.seen_f3 || !state.seen_f4 {
        100
    } else {
        0
    }
}

fn main() {
    let mut state = State {
        seen_f1: false,
        seen_f2: false,
        seen_f3: false,
        seen_f4: false,
        log: Vec::new(),
    };

    let pf: Vec<fn(&mut State, i32, i32) -> i32> = vec![target];

    let idx = f1(&mut state);
    let rc = pf[idx as usize](&mut state, f2(&mut state), f3(&mut state) + f4(&mut state));
    if rc != 0 {
        std::process::exit(rc);
    }

    if state.log.len() != 5 {
        std::process::exit(1);
    }
    if state.log[4] != 9 {
        std::process::exit(2);
    }

    let mut counts = [0; 10];
    for &id in &state.log[0..4] {
        counts[id as usize] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        std::process::exit(3);
    }

    std::process::exit(0);
}