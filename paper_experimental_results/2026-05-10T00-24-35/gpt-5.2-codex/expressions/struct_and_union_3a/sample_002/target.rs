use std::process::exit;

struct U {
    type_field: i32,
    doublenode: f64,
}

impl U {
    fn new() -> Self {
        Self {
            type_field: 0,
            doublenode: 0.0,
        }
    }

    fn set_type(&mut self, v: i32) {
        self.type_field = v;
    }

    fn set_double(&mut self, v: f64) {
        self.doublenode = v;
    }

    fn n_alltypes(&self) -> i32 {
        self.type_field
    }

    fn ni_type(&self) -> i32 {
        self.type_field
    }

    fn nf_type(&self) -> i32 {
        self.type_field
    }

    fn nf_doublenode(&self) -> f64 {
        self.doublenode
    }
}

fn main() {
    let mut u = U::new();

    u.set_type(1);
    u.set_double(3.14);

    if u.n_alltypes() != 1 {
        exit(1);
    }
    if u.ni_type() != 1 {
        exit(1);
    }
    if u.nf_type() != 1 {
        exit(2);
    }
    if u.nf_doublenode() != 3.14 {
        exit(3);
    }

    exit(0);
}