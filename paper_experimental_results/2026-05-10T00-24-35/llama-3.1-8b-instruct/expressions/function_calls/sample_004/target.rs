fn main() {
    let mut logv = vec![0; 5];
    let mut logn = 0;

    fn log_call(id: i32) {
        if logn < logv.len() {
            logv[logn] = id;
            logn += 1;
        }
    }

    let mut seen_f1 = false;
    let mut seen_f2 = false;
    let mut seen_f3 = false;
    let mut seen_f4 = false;

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

        if !seen_f1 || !seen_f2 || !seen_f3 || !seen_f4 {
            return 100;
        }

        0
    }

    let pf = [target];
    let rc = pf[f1()](f2(), f3() + f4());
    if rc != 0 {
        return;
    }

    if logn != 5 {
        eprintln!("logn != 5");
        return;
    }
    if logv[4] != 9 {
        eprintln!("logv[4] != 9");
        return;
    }

    let counts = vec![0; 10];
    for i in 0..4 {
        counts[logv[i]] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        eprintln!("counts incorrect");
        return;
    }

    println!("0");
}