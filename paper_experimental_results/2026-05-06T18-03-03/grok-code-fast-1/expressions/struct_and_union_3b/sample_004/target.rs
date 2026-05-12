struct U12 {
    m: i32,
}

impl U12 {
    fn s1(&self) -> &i32 {
        &self.m
    }
    fn s1_mut(&mut self) -> &mut i32 {
        &mut self.m
    }
    fn s2(&self) -> &i32 {
        &self.m
    }
    fn s2_mut(&mut self) -> &mut i32 {
        &mut self.m
    }
}

fn f(u: &mut U12) -> i32 {
    if *u.s1() < 0 {
        *u.s2_mut() = -*u.s2();
    }
    *u.s1()
}

fn main() {
    let mut u = U12 { m: 0 };

    *u.s1_mut() = -7;
    *u.s2_mut() = 7;

    if f(&mut u) != -7 {
        std::process::exit(1);
    }
    if *u.s2() != -7 {
        std::process::exit(2);
    }

    std::process::exit(0);
}