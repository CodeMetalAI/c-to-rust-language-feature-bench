fn main() {
    static mut SEEN_F1: bool = false;
    static mut SEEN_F2: bool = false;
    static mut SEEN_F3: bool = false;
    static mut SEEN_F4: bool = false;
    static mut LOGV: [i32; 5] = [0; 5];
    static mut LOGN: usize = 0;

    unsafe fn log_call(id: i32) {
        if LOGN < 5 {
            LOGV[LOGN] = id;
            LOGN += 1;
        }
    }

    unsafe fn f1() -> i32 {
        SEEN_F1 = true;
        log_call(1);
        0
    }
    unsafe fn f2() -> i32 {
        SEEN_F2 = true;
        log_call(2);
        20
    }
    unsafe fn f3() -> i32 {
        SEEN_F3 = true;
        log_call(3);
        30
    }
    unsafe fn f4() -> i32 {
        SEEN_F4 = true;
        log_call(4);
        40
    }
    unsafe fn target(a: i32, b: i32) -> i32 {
        (a, b); // suppress unused variable warnings
        log_call(9);

        if !(SEEN_F1 && SEEN_F2 && SEEN_F3 && SEEN_F4) {
            return 100;
        }

        0
    }

    let pf: [fn(i32, i32) -> i32; 1] = [target];

    let rc = unsafe {
        pf[f1() as usize](f2(), f3() + f4())
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

        let mut counts: [i32; 10] = [0; 10];
        for &item in LOGV.iter().take(4) {
            counts[item as usize] += 1;
        }

        if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
            std::process::exit(3);
        }
    }

    std::process::exit(0);
}