fn main() {
    static mut SEEN_F1: bool = false;
    static mut SEEN_F2: bool = false;
    static mut SEEN_F3: bool = false;
    static mut SEEN_F4: bool = false;
    static mut LOGV: [i32; 5] = [0; 5];
    static mut LOGN: usize = 0;

    fn log_call(id: i32) {
        unsafe {
            if LOGN < 5 {
                LOGV[LOGN] = id;
                LOGN += 1;
            }
        }
    }

    fn f1() -> i32 {
        unsafe {
            SEEN_F1 = true;
        }
        log_call(1);
        0
    }

    fn f2() -> i32 {
        unsafe {
            SEEN_F2 = true;
        }
        log_call(2);
        20
    }

    fn f3() -> i32 {
        unsafe {
            SEEN_F3 = true;
        }
        log_call(3);
        30
    }

    fn f4() -> i32 {
        unsafe {
            SEEN_F4 = true;
        }
        log_call(4);
        40
    }

    fn target(a: i32, b: i32) -> i32 {
        let _ = a;
        let _ = b;
        log_call(9);

        unsafe {
            if !SEEN_F1 || !SEEN_F2 || !SEEN_F3 || !SEEN_F4 {
                return 100;
            }
        }

        0
    }

    let pf: [fn(i32, i32) -> i32; 1] = [target];

    let rc = pf[f1() as usize](f2(), f3() + f4());
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
            counts[LOGV[i] as usize] += 1;
        }

        if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
            std::process::exit(3);
        }
    }
}