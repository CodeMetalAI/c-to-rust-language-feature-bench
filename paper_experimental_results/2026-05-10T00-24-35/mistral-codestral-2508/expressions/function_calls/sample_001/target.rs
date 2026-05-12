fn main() {
    let mut seen_f1 = false;
    let mut seen_f2 = false;
    let mut seen_f3 = false;
    let mut seen_f4 = false;
    let mut logv = Vec::new();
    let mut logn = 0;

    fn log_call(logv: &mut Vec<i32>, logn: &mut usize, id: i32) {
        if *logn < 5 {
            logv.push(id);
            *logn += 1;
        }
    }

    fn f1(seen_f1: &mut bool, logv: &mut Vec<i32>, logn: &mut usize) -> i32 {
        *seen_f1 = true;
        log_call(logv, logn, 1);
        0
    }

    fn f2(seen_f2: &mut bool, logv: &mut Vec<i32>, logn: &mut usize) -> i32 {
        *seen_f2 = true;
        log_call(logv, logn, 2);
        20
    }

    fn f3(seen_f3: &mut bool, logv: &mut Vec<i32>, logn: &mut usize) -> i32 {
        *seen_f3 = true;
        log_call(logv, logn, 3);
        30
    }

    fn f4(seen_f4: &mut bool, logv: &mut Vec<i32>, logn: &mut usize) -> i32 {
        *seen_f4 = true;
        log_call(logv, logn, 4);
        40
    }

    fn target(_a: i32, _b: i32, logv: &mut Vec<i32>, logn: &mut usize, seen_f1: bool, seen_f2: bool, seen_f3: bool, seen_f4: bool) -> i32 {
        log_call(logv, logn, 9);

        if !seen_f1 || !seen_f2 || !seen_f3 || !seen_f4 {
            100
        } else {
            0
        }
    }

    let pf = [target];

    let rc = pf[f1(&mut seen_f1, &mut logv, &mut logn) as usize](
        f2(&mut seen_f2, &mut logv, &mut logn),
        f3(&mut seen_f3, &mut logv, &mut logn) + f4(&mut seen_f4, &mut logv, &mut logn),
        &mut logv,
        &mut logn,
        seen_f1,
        seen_f2,
        seen_f3,
        seen_f4,
    );

    if rc != 0 {
        return;
    }

    if logn != 5 {
        return;
    }

    if logv[4] != 9 {
        return;
    }

    let mut counts = [0; 10];
    for i in 0..4 {
        counts[logv[i] as usize] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        return;
    }
}