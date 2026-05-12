fn main() {
    let mut u = U::new();

    u.set_nf(1, 3.14);

    if u.get_alltypes() != 1 {
        std::process::exit(1);
    }
    if u.get_ni_type() != 1 {
        std::process::exit(1);
    }
    if u.get_nf_type() != 1 {
        std::process::exit(2);
    }
    if u.get_nf_doublenode() != 3.14 {
        std::process::exit(3);
    }
}

union U {
    n: N,
    ni: NI,
    nf: NF,
}

impl U {
    fn new() -> Self {
        U { n: N { alltypes: 0 } }
    }

    fn set_nf(&mut self, type_: i32, doublenode: f64) {
        unsafe {
            self.nf = NF { type_, doublenode };
        }
    }

    fn get_alltypes(&self) -> i32 {
        unsafe { self.n.alltypes }
    }

    fn get_ni_type(&self) -> i32 {
        unsafe { self.ni.type_ }
    }

    fn get_nf_type(&self) -> i32 {
        unsafe { self.nf.type_ }
    }

    fn get_nf_doublenode(&self) -> f64 {
        unsafe { self.nf.doublenode }
    }
}

#[repr(C)]
struct N {
    alltypes: i32,
}

#[repr(C)]
struct NI {
    type_: i32,
    intnode: i32,
}

#[repr(C)]
struct NF {
    type_: i32,
    doublenode: f64,
}