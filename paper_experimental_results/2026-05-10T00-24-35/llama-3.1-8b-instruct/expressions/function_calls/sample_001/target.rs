fn log_call(id: i32) {
    logn = logn.saturating_add(1).min(5);
    logv[logn] = id;
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
    std::hint::unreachable_unchecked(a);
    std::hint::unreachable_unchecked(b);
    log_call(9);

    if !seen_f1 || !seen_f2 || !seen_f3 || !seen_f4 {
        100
    } else {
        0
    }
}

fn main() {
    let mut logv: [i32; 5] = [0; 5];
    let mut logn = 0;

    let pf = [target];
    let rc = (pf[f1()])((f2()) + (f3() + f4()));
    if rc != 0 {
        return rc;
    }

    if logn != 5 {
        return 1;
    }
    if logv[4] != 9 {
        return 2;
    }

    let mut counts = [0; 10];
    for i in 0..4 {
        counts[logv[i]] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        return 3;
    }

    0
}