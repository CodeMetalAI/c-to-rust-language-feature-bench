fn main() {
    let mut seen = [false; 4];
    let mut log: Vec<i32> = Vec::new();

    let f1 = || {
        seen[0] = true;
        log.push(1);
        0
    };
    let f2 = || {
        seen[1] = true;
        log.push(2);
        20
    };
    let f3 = || {
        seen[2] = true;
        log.push(3);
        30
    };
    let f4 = || {
        seen[3] = true;
        log.push(4);
        40
    };

    let target = |_: i32, _: i32| {
        log.push(9);
        if seen.iter().all(|&x| x) {
            0
        } else {
            100
        }
    };

    let pf: Vec<Box<dyn Fn(i32, i32) -> i32>> = vec![Box::new(target)];

    let rc = pf[f1() as usize](f2(), f3() + f4());
    if rc != 0 {
        std::process::exit(rc);
    }

    if log.len() != 5 {
        std::process::exit(1);
    }
    if log[4] != 9 {
        std::process::exit(2);
    }

    let mut counts = [0; 10];
    for i in 0..4 {
        counts[log[i] as usize] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        std::process::exit(3);
    }

    std::process::exit(0);
}