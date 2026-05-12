fn main() {
    let mut seen_f1 = false;
    let mut seen_f2 = false;
    let mut seen_f3 = false;
    let mut seen_f4 = false;
    let mut logv = [0; 5];
    let mut logn = 0;

    fn log_call(id: i32) {
        if logn < 5 {
            logv[logn as usize] = id;
            logn += 1;
        }
    }

    fn f1() -> i32 {
        seen_f1 = true;
        log_call(1);
        0
    }

    fn f2() -> i32 {
        seen_f2 = true;
        log_call(2);
        20
    }

    fn f3() -> i32 {
        seen_f3 = true;
        log_call(3);
        30
    }

    fn f4() -> i32 {
        seen_f4 = true;
        log_call(4);
        40
    }

    fn target(a: i32, b: i32) -> i32 {
        log_call(9);

        if!seen_f1 ||!seen_f2 ||!seen_f3 ||!seen_f4 {
            100
        } else {
            0
        }
    }

    let pf: [fn(i32, i32) -> i32; 1] = [target];

    let rc = pf[f1() as usize](f2(), f3() + f4());
    if rc!= 0 {
        std::process::exit(rc);
    }

    if logn!= 5 {
        std::process::exit(1);
    }
    if logv[4]!= 9 {
        std::process::exit(2);
    }

    let mut counts = [0; 10];
    for i in 0..4 {
        counts[logv[i] as usize] += 1;
    }

    if counts[1]!= 1 || counts[2]!= 1 || counts[3]!= 1 || counts[4]!= 1 {
        std::process::exit(3);
    }

    std::process::exit(0);
}