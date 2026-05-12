struct T1 {
    m: i32,
}

struct T2 {
    m: i32,
}

struct U12 {
    value: i32,
}

impl U12 {
    fn s1_m(&self) -> i32 {
        self.value
    }
    fn s1_m_mut(&mut self) -> &mut i32 {
        &mut self.value
    }
    fn s2_m(&self) -> i32 {
        self.value
    }
    fn s2_m_mut(&mut self) -> &mut i32 {
        &mut self.value
    }
}

fn f(u: &mut U12) -> i32 {
    if u.s1_m() < 0 {
        *u.s2_m_mut() = -u.s2_m();
    }
    u.s1_m()
}

fn main() {
    let mut u = U12 { value: 0 };

    *u.s1_m_mut() = -7;
    *u.s2_m_mut() = 7;

    if f(&mut u) != -7 {
        std::process::exit(1);
    }
    if u.s2_m() != -7 {
        std::process::exit(2);
    }
    std::process::exit(0);
}