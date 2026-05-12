struct U12 {
    m: i32,
}

impl U12 {
    fn set_s1(&mut self, v: i32) {
        self.m = v;
    }
    fn set_s2(&mut self, v: i32) {
        self.m = v;
    }
    fn get_s1(&self) -> i32 {
        self.m
    }
    fn get_s2(&self) -> i32 {
        self.m
    }
}

fn f(u: &mut U12) -> i32 {
    if u.get_s1() < 0 {
        let v = u.get_s2();
        u.set_s2(-v);
    }
    u.get_s1()
}

fn main() {
    let mut u = U12 { m: 0 };

    u.set_s1(-7);
    u.set_s2(7);

    if f(&mut u) != -7 {
        std::process::exit(1);
    }
    if u.get_s2() != -7 {
        std::process::exit(2);
    }

    std::process::exit(0);
}