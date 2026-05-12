#[derive(Copy, Clone)]
struct IntList {
    car: i32,
    cdr: Cdr,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cdr {
    Endless,
}

impl Cdr {
    fn cdr(&self) -> Cdr {
        *self
    }
}

static ENDLESS_ZEROS: IntList = IntList {
    car: 0,
    cdr: Cdr::Endless,
};

fn eval(x: IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if x.cdr != Cdr::Endless {
        return 2;
    }
    if x.cdr.cdr() != Cdr::Endless {
        return 3;
    }
    0
}

fn main() {
    let code = eval(ENDLESS_ZEROS);
    std::process::exit(code);
}