struct Logger {
    seen_f1: bool,
    seen_f2: bool,
    seen_f3: bool,
    seen_f4: bool,
    logv: [i32; 5],
    logn: usize,
}

impl Logger {
    fn new() -> Self {
        Logger {
            seen_f1: false,
            seen_f2: false,
            seen_f3: false,
            seen_f4: false,
            logv: [0; 5],
            logn: 0,
        }
    }

    fn log_call(&mut self, id: i32) {
        if self.logn < 5 {
            self.logv[self.logn] = id;
            self.logn += 1;
        }
    }

    fn f1(&mut self) -> i32 {
        self.seen_f1 = true;
        self.log_call(1);
        0
    }

    fn f2(&mut self) -> i32 {
        self.seen_f2 = true;
        self.log_call(2);
        20
    }

    fn f3(&mut self) -> i32 {
        self.seen_f3 = true;
        self.log_call(3);
        30
    }

    fn f4(&mut self) -> i32 {
        self.seen_f4 = true;
        self.log_call(4);
        40
    }

    fn target(&mut self, _a: i32, _b: i32) -> i32 {
        self.log_call(9);
        if !self.seen_f1 || !self.seen_f2 || !self.seen_f3 || !self.seen_f4 {
            return 100;
        }
        0
    }
}

fn main() {
    let mut logger = Logger::new();

    let rc = (logger.target)(logger.f1(), logger.f2() + logger.f3() + logger.f4());
    if rc != 0 {
        std::process::exit(rc);
    }

    if logger.logn != 5 {
        std::process::exit(1);
    }
    if logger.logv[4] != 9 {
        std::process::exit(2);
    }

    let mut counts = [0; 10];
    for i in 0..4 {
        counts[logger.logv[i] as usize] += 1;
    }

    if counts[1] != 1 || counts[2] != 1 || counts[3] != 1 || counts[4] != 1 {
        std::process::exit(3);
    }

    std::process::exit(0);
}