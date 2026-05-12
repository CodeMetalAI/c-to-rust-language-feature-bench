static mut SEEN_F1: i32 = 0;
static mut SEEN_F2: i32 = 0;
static mut SEEN_F3: i32 = 0;
static mut SEEN_F4: i32 = 0;
static mut LOGV: [i32; 5] = [0; 5];
static mut LOGN: i32 = 0;

fn log_call(id: i32) {
    unsafe {
        if LOGN < 5 {
            LOGV[LOGN as usize] = id;
            LOGN += 1;
        }
    }
}

fn f1() -> i32 {
    unsafe {
        SEEN_F1 = 1;
        log_call(1);
    }
    0
}
fn f2() -> i32 {
    unsafe {
        SEEN_F2 = 1;
        log_call(2);
    }
    20
}
fn f3() -> i32 {
    unsafe {
        SEEN_F3 = 1;
        log_call(3);
    }
    30
}
fn f4() -> i32 {
    unsafe {
        SEEN_F4 = 1;
        log_call(4);
    }
    40
}

type Fn2 = fn(i32, i32) -> i32;

fn target(a: i32, b: i32) -> i32 {
    unsafe {
        log_call(9);
    }
    if SEEN_F1 == 0 || SEEN_F2 == 0 || SEEN_F3 == 0 || SEEN_F4 == 0 {
        100
    } else {
        0
    }
}

fn main() -> i32 {
    let pf: [Fn2; 1] = [target];

    let rc = unsafe { (*pf[f1()])(f2(), f3() + f4()) };

    if rc != 0 {
        rc
    } else {
        unsafe {
            if LOGN != 5 {
                1
            } else if LOGV[4] != 9 {
                2
            } else {
                let mut counts = [0; 10];
                for i in 0..4 {
                    counts[LOGV[i] as usize] += 1;
                }

                if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
                    3
                } else {
                    0
                }
            }
        }
    }
}