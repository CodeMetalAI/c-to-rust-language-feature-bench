fn main() {
    let mut seen_f1 = false;
    let mut seen_f2 = false;
    let mut seen_f3 = false;
    let mut seen_f4 = false;
    let mut logv: Vec<i32> = Vec::with_capacity(5);

    let log_call = |id: i32| {
        if logv.len() < 5 {
            logv.push(id);
        }
    };

    let f1 = || -> i32 {
        seen_f1 = true;
        log_call(1);
        0
    };

    let f2 = || -> i32 {
        seen_f2 = true;
        log_call(2);
        20
    };

    let f3 = || -> i32 {
        seen_f3 = true;
        log_call(3);
        30
    };

    let f4 = || -> i32 {
        seen_f4 = true;
        log_call(4);
        40
    };

    let target = |_: i32, _: i32| -> i32 {
        log_call(9);
        if !seen_f1 || !seen_f2 || !seen_f3 || !seen_f4 {
            100
        } else {
            0
        }
    };

    let pf: Vec<Box<dyn Fn(i32, i32) -> i32>> = vec![Box::new(target)];

    let rc = pf[f1() as usize](f2(), f3() + f4());
    if rc != 0 {
        std::process::exit(rc);
    }

    if logv.len() != 5 {
        std::process::exit(1);
    }
    if logv[4] != 9 {
        std::process::exit(2);
    }

    let mut counts = [0; 10];
    for &id in &logv[0..4] {
        counts[id as usize] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        std::process::exit(3);
    }

    std::process::exit(0);
}