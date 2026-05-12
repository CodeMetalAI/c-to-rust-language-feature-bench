#[derive(Clone, Copy)]
struct S {
    i: f64,
}

const SIZE: usize = std::mem::size_of::<i32>() + std::mem::size_of::<f64>();

struct U([u8; SIZE]);

impl U {
    fn set_u1_f2_i(&mut self, val: f64) {
        let bytes = val.to_ne_bytes();
        self.0[4..12].copy_from_slice(&bytes);
    }

    fn get_u1_f2_i(&self) -> f64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.0[4..12]);
        f64::from_ne_bytes(bytes)
    }

    fn set_u2_f3_i(&mut self, val: f64) {
        let bytes = val.to_ne_bytes();
        self.0[0..8].copy_from_slice(&bytes);
    }

    fn get_u2_f3_i(&self) -> f64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.0[0..8]);
        f64::from_ne_bytes(bytes)
    }

    fn get_u1_f2(&self) -> S {
        S { i: self.get_u1_f2_i() }
    }

    fn set_u1_f2(&mut self, s: S) {
        self.set_u1_f2_i(s.i);
    }

    fn set_u2_f3(&mut self, s: S) {
        self.set_u2_f3_i(s.i);
    }
}

fn f(g: &U) -> S {
    g.get_u1_f2()
}

fn foo() -> i32 {
    1
}

fn main() {
    let mut g = U([0u8; SIZE]);
    g.set_u1_f2_i(1.0);
    g.set_u2_f3_i(1.0);
    g.set_u1_f2(f(&g));
    g.set_u2_f3(f(&g));
    let val = foo() as f64;
    let sum = g.get_u1_f2_i() + g.get_u2_f3_i() + val;
    std::process::exit(if (sum - 3.0).abs() < f64::EPSILON { 0 } else { 1 });
}