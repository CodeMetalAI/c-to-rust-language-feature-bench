static SEEN_F1: std::sync::Mutex<bool> = std::sync::Mutex::new(false);
static SEEN_F2: std::sync::Mutex<bool> = std::sync::Mutex::new(false);
static SEEN_F3: std::sync::Mutex<bool> = std::sync::Mutex::new(false);
static SEEN_F4: std::sync::Mutex<bool> = std::sync::Mutex::new(false);

static LOGV: std::sync::Mutex<[i32; 5]> = std::sync::Mutex::new([0; 5]);
static LOGN: std::sync::Mutex<usize> = std::sync::Mutex::new(0);

fn log_call(id: i32) {
    let mut logn = LOGN.lock().unwrap();
    if *logn < 5 {
        let mut logv = LOGV.lock().unwrap();
        logv[*logn] = id;
        *logn += 1;
    }
}

fn f1() -> i32 {
    let mut seen_f1 = SEEN_F1.lock().unwrap();
    *seen_f1 = true;
    log_call(1);
    0
}

fn f2() -> i32 {
    let mut seen_f2 = SEEN_F2.lock().unwrap();
    *seen_f2 = true;
    log_call(2);
    20
}

fn f3() -> i32 {
    let mut seen_f3 = SEEN_F3.lock().unwrap();
    *seen_f3 = true;
    log_call(3);
    30
}

fn f4() -> i32 {
    let mut seen_f4 = SEEN_F4.lock().unwrap();
    *seen_f4 = true;
    log_call(4);
    40
}

type Fn2 = fn(i32, i32) -> i32;

fn target(a: i32, b: i32) -> i32 {
    log_call(9);

    let seen_f1 = SEEN_F1.lock().unwrap();
    let seen_f2 = SEEN_F2.lock().unwrap();
    let seen_f3 = SEEN_F3.lock().unwrap();
    let seen_f4 = SEEN_F4.lock().unwrap();

    if !*seen_f1 || !*seen_f2 || !*seen_f3 || !*seen_f4 {
        return 100;
    }

    0
}

fn main() -> i32 {
    let pf: [Fn2; 1] = [target];

    let rc = pf[0](f1(), f2() + f3() + f4());
    if rc != 0 {
        return rc;
    }

    let logn = LOGN.lock().unwrap();
    if *logn != 5 {
        return 1;
    }
    let logv = LOGV.lock().unwrap();
    if logv[4] != 9 {
        return 2;
    }

    let mut counts = [0; 10];
    for i in 0..4 {
        counts[logv[i] as usize] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        return 3;
    }

    0
}