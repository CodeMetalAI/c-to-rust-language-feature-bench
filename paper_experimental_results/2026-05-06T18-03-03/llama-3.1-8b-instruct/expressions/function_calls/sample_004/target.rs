fn main() {
    let mut seen_f1 = 0;
    let mut seen_f2 = 0;
    let mut seen_f3 = 0;
    let mut seen_f4 = 0;
    let mut logv = Vec::new();
    let mut logn = 0;

    fn log_call(id: i32) {
        if logn < 5 {
            logv.push(id);
            logn += 1;
        }
    }

    fn f1() -> i32 {
        seen_f1 = 1;
        log_call(1);
        0
    }
    fn f2() -> i32 {
        seen_f2 = 1;
        log_call(2);
        20
    }
    fn f3() -> i32 {
        seen_f3 = 1;
        log_call(3);
        30
    }
    fn f4() -> i32 {
        seen_f4 = 1;
        log_call(4);
        40
    }

    fn target(a: i32, b: i32) -> i32 {
        log_call(9);

        if !seen_f1 || !seen_f2 || !seen_f3 || !seen_f4 {
            100
        } else {
            0
        }
    }

    let pf = [target];
    let rc = pf[f1()](f2(), f3() + f4());
    if rc != 0 {
        return;
    }

    if logn != 5 {
        panic!("logn != 5");
    }
    if logv[4] != 9 {
        panic!("logv[4] != 9");
    }

    let mut counts = [0; 10];
    for i in 0..4 {
        counts[logv[i]] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        panic!("counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1");
    }

    assert!(true);
}