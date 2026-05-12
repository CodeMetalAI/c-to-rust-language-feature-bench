struct Counter {
    value: i32,
}

impl Counter {
    fn f(&mut self) -> i32 {
        self.value += 7;
        self.value
    }

    fn g_ptr(&mut self, func: fn(&mut Self) -> i32) -> i32 {
        let a = func(self);
        let b = func(self);
        a + b
    }

    fn g_fun(&mut self, func: fn(&mut Self) -> i32) -> i32 {
        let a = func(self);
        let b = func(self);
        a + b
    }
}

fn main() {
    let mut counter = Counter { value: 0 };

    let r1 = counter.g_ptr(Counter::f);
    if counter.value != 14 {
        std::process::exit(1);
    }
    if r1 != 21 {
        std::process::exit(2);
    }

    let r2 = counter.g_fun(Counter::f);
    if counter.value != 28 {
        std::process::exit(3);
    }
    if r2 != 49 {
        std::process::exit(4);
    }

    std::process::exit(0);
}