use std::cell::RefCell;

#[derive(Debug)]
struct Logger {
    seen_f1: bool,
    seen_f2: bool,
    seen_f3: bool,
    seen_f4: bool,
    logv: Vec<i32>,
    logn: usize,
}

impl Logger {
    fn new() -> Self {
        Logger {
            seen_f1: false,
            seen_f2: false,
            seen_f3: false,
            seen_f4: false,
            logv: Vec::with_capacity(5),
            logn: 0,
        }
    }

    fn log_call(&mut self, id: i32) {
        if self.logn < 5 {
            self.logv.push(id);
            self.logn += 1;
        }
    }

    fn all_seen(&self) -> bool {
        self.seen_f1 && self.seen_f2 && self.seen_f3 && self.seen_f4
    }
}

fn f1(logger: &mut Logger) -> i32 {
    logger.seen_f1 = true;
    logger.log_call(1);
    0
}

fn f2(logger: &mut Logger) -> i32 {
    logger.seen_f2 = true;
    logger.log_call(2);
    20
}

fn f3(logger: &mut Logger) -> i32 {
    logger.seen_f3 = true;
    logger.log_call(3);
    30
}

fn f4(logger: &mut Logger) -> i32 {
    logger.seen_f4 = true;
    logger.log_call(4);
    40
}

fn target(logger: &mut Logger, _a: i32, _b: i32) -> i32 {
    logger.log_call(9);
    if !logger.all_seen() {
        100
    } else {
        0
    }
}

fn main() -> i32 {
    let mut logger = Logger::new();
    let pf: Vec<fn(&mut Logger, i32, i32) -> i32> = vec![target];

    let idx = f1(&mut logger) as usize;
    let rc = pf[idx](&mut logger, f2(&mut logger), f3(&mut logger) + f4(&mut logger));
    if rc != 0 {
        return rc;
    }

    if logger.logn != 5 {
        return 1;
    }
    if logger.logv[4] != 9 {
        return 2;
    }

    let mut counts: [i32; 10] = [0; 10];
    for &id in &logger.logv[..4] {
        counts[id as usize] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        return 3;
    }

    0
}