use std::cell::Cell;

static mut SEEN_F1: Cell<bool> = Cell::new(false);
static mut SEEN_F2: Cell<bool> = Cell::new(false);
static mut SEEN_F3: Cell<bool> = Cell::new(false);
static mut SEEN_F4: Cell<bool> = Cell::new(false);
static mut LOGV: [Cell<i32>; 5] = [Cell::new(0); 5];
static mut LOGN: Cell<i32> = Cell::new(0);

fn log_call(id: i32) {
    unsafe {
        if LOGN.get() < 5 {
            LOGV[LOGN.get() as usize].set(id);
            LOGN.set(LOGN.get() + 1);
        }
    }
}

fn f1() -> i32 {
    unsafe {
        SEEN_F1.set(true);
        log_call(1);
    }
    0
}

fn f2() -> i32 {
    unsafe {
        SEEN_F2.set(true);
        log_call(2);
    }
    20
}

fn f3() -> i32 {
    unsafe {
        SEEN_F3.set(true);
        log_call(3);
    }
    30
}

fn f4() -> i32 {
    unsafe {
        SEEN_F4.set(true);
        log_call(4);
    }
    40
}

type Fn2T = fn(i32, i32) -> i32;

fn target(a: i32, b: i32) -> i32 {
    log_call(9);
    unsafe {
        if !(SEEN_F1.get() && SEEN_F2.get() && SEEN_F3.get() && SEEN_F4.get()) {
            return 100;
        }
    }
    0
}

fn main() {
    let pf: [Fn2T; 1] = [target];

    let rc = pf[f1() as usize](f2(), f3() + f4());
    if rc != 0 {
        std::process::exit(rc);
    }

    unsafe {
        if LOGN.get() != 5 {
            std::process::exit(1);
        }
        if LOGV[4].get() != 9 {
            std::process::exit(2);
        }

        let mut counts = [0; 10];
        for i in 0..4 {
            counts[LOGV[i].get() as usize] += 1;
        }

        if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
            std::process::exit(3);
        }
    }

    std::process::exit(0);
}