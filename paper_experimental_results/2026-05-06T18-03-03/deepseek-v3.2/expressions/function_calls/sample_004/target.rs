static mut SEEN_F1: i32 = 0;
static mut SEEN_F2: i32 = 0;
static mut SEEN_F3: i32 = 0;
static mut SEEN_F4: i32 = 0;
static mut LOGV: [i32; 5] = [0; 5];
static mut LOGN: i32 = 0;

unsafe fn log_call(id: i32) {
    if LOGN < 5 {
        LOGV[LOGN as usize] = id;
        LOGN += 1;
    }
}

unsafe fn f1() -> i32 {
    SEEN_F1 = 1;
    log_call(1);
    0
}

unsafe fn f2() -> i32 {
    SEEN_F2 = 1;
    log_call(2);
    20
}

unsafe fn f3() -> i32 {
    SEEN_F3 = 1;
    log_call(3);
    30
}

unsafe fn f4() -> i32 {
    SEEN_F4 = 1;
    log_call(4);
    40
}

type Fn2 = fn(i32, i32) -> i32;

unsafe fn target(a: i32, b: i32) -> i32 {
    let _ = a;
    let _ = b;
    log_call(9);

    if SEEN_F1 == 0 || SEEN_F2 == 0 || SEEN_F3 == 0 || SEEN_F4 == 0 {
        return 100;
    }

    0
}

fn main() {
    let rc = unsafe {
        let pf: [Fn2; 1] = [target];
        let idx = f1() as usize;
        let arg1 = f2();
        let arg2 = f3() + f4();
        pf[idx](arg1, arg2)
    };

    if rc != 0 {
        std::process::exit(rc);
    }

    unsafe {
        if LOGN != 5 {
            std::process::exit(1);
        }
        if LOGV[4] != 9 {
            std::process::exit(2);
        }

        let mut counts = [0; 10];
        for i in 0..4 {
            counts[LOGV[i as usize] as usize] += 1;
        }

        if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
            std::process::exit(3);
        }
    }

    std::process::exit(0);
}