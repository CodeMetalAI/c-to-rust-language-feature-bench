fn main() {
    let mut seen_f1 = false;
    let mut seen_f2 = false;
    let mut seen_f3 = false;
    let mut seen_f4 = false;
    let mut logv = vec![0; 5];
    let mut logn = 0;
    
    fn log_call(id: i32, logv: &mut Vec<i32>, logn: &mut usize) {
        if *logn < 5 {
            logv[*logn] = id;
            *logn += 1;
        }
    }

    fn f1(seen_f1: &mut bool, logv: &mut Vec<i32>, logn: &mut usize) -> i32 {
        *seen_f1 = true;
        log_call(1, logv, logn);
        0
    }
    fn f2(seen_f2: &mut bool, logv: &mut Vec<i32>, logn: &mut usize) -> i32 {
        *seen_f2 = true;
        log_call(2, logv, logn);
        20
    }
    fn f3(seen_f3: &mut bool, logv: &mut Vec<i32>, logn: &mut usize) -> i32 {
        *seen_f3 = true;
        log_call(3, logv, logn);
        30
    }
    fn f4(seen_f4: &mut bool, logv: &mut Vec<i32>, logn: &mut usize) -> i32 {
        *seen_f4 = true;
        log_call(4, logv, logn);
        40
    }

    fn target(
        _a: i32,
        _b: i32,
        seen_f1: &bool,
        seen_f2: &bool,
        seen_f3: &bool,
        seen_f4: &bool,
        logv: &mut Vec<i32>,
        logn: &mut usize
    ) -> i32 {
        log_call(9, logv, logn);

        if !seen_f1 || !seen_f2 || !seen_f3 || !seen_f4 {
            100
        } else {
            0
        }
    }

    let pf: [fn(i32, i32, &bool, &bool, &bool, &bool, &mut Vec<i32>, &mut usize) -> i32; 1] = [target];

    let rc = pf[f1(&mut seen_f1, &mut logv, &mut logn) as usize](
        f2(&mut seen_f2, &mut logv, &mut logn),
        f3(&mut seen_f3, &mut logv, &mut logn) + f4(&mut seen_f4, &mut logv, &mut logn),
        &seen_f1,
        &seen_f2,
        &seen_f3,
        &seen_f4,
        &mut logv,
        &mut logn,
    );
    
    if rc != 0 {
        std::process::exit(rc);
    }

    if logn != 5 {
        std::process::exit(1);
    }
    if logv[4] != 9 {
        std::process::exit(2);
    }

    let mut counts = vec![0; 10];
    for &val in &logv[..4] {
        counts[val as usize] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        std::process::exit(3);
    }
}