struct T1 {
    m: i32,
}

struct T2 {
    m: i32,
}

struct U12 {
    m: i32,
}

impl U12 {
    fn s1(&self) -> T1 {
        T1 { m: self.m }
    }
    fn s2(&self) -> T2 {
        T2 { m: self.m }
    }
    fn set_s1_m(&mut self, val: i32) {
        self.m = val;
    }
    fn set_s2_m(&mut self, val: i32) {
        self.m = val;
    }
}

fn f(u: &mut U12) -> i32 {
    if u.s1().m < 0 {
        let new_val = -u.s2().m;
        u.set_s2_m(new_val);
    }
    u.s1().m
}

fn main() {
    let mut u = U12 { m: 0 };

    u.set_s1_m(-7);
    u.set_s2_m(7);

    if f(&mut u) != -7 {
        std::process::exit(1);
    }
    if u.s2().m != -7 {
        std::process::exit(2);
    }

    std::process::exit(0);
}