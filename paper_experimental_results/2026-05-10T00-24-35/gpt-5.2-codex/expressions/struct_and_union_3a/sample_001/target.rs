struct U {
    alltypes: i32,
    ni_type: i32,
    intnode: i32,
    nf_type: i32,
    doublenode: f64,
}

impl U {
    fn new() -> Self {
        Self {
            alltypes: 0,
            ni_type: 0,
            intnode: 0,
            nf_type: 0,
            doublenode: 0.0,
        }
    }

    fn set_nf_type(&mut self, v: i32) {
        self.alltypes = v;
        self.ni_type = v;
        self.nf_type = v;
    }

    fn set_nf_doublenode(&mut self, v: f64) {
        self.doublenode = v;
    }
}

fn main() {
    let mut u = U::new();

    u.set_nf_type(1);
    u.set_nf_doublenode(3.14);

    if u.alltypes != 1 {
        std::process::exit(1);
    }
    if u.ni_type != 1 {
        std::process::exit(1);
    }
    if u.nf_type != 1 {
        std::process::exit(2);
    }
    if u.doublenode != 3.14 {
        std::process::exit(3);
    }
}